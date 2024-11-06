/*
 * 1. RefCell provides interior mutability, but allows borrowing at runtime using
 * borrow, borrow_mut, replace method.
 * 2. Allows you to mutate even when the data itself is behind the immutable reference.
 *    Example:
 *    Struct Counter is immutable, As per external mutablity (aka inheritered 
 *    mutability, variables in the structure is also immutable.
 *    However, only the count can be made mutable using RefCell.
 * 3. Thread safety is not available. So used only in Single threaded application.
 * 4. Borrow checking is done at runtime. So compilation will succeed, but will
 * cause panic during runtime.
 * 5. Rules:
 *    1. One mutable reference can exist at any point in time.
 *    2. Several immutable reference can exists, but there should not be any mutable
 *    references.
 *    For example:
 *    Below code will result in panic with
 *
 *  thread 'main' panicked at 'already mutably borrowed: BorrowError', refcell.rs:55:28
 *
 *  As _x has borrow_mut and _y has borrow immutable.
 *
 *  6. Used very much in Tree/Linkedlist implementation
 */

use std::cell::RefCell;

struct Counter {
    value: RefCell<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            value: RefCell::new(0),
        }
    }

    fn increment(&self) {
        let mut val = self.value.borrow_mut();
        *val += 1;
    }

    fn get_value(&self) -> i32 {
        *self.value.borrow()
    }
}

fn main() {
    let counter = Counter::new();
    counter.increment();
    counter.increment();
    println!("Counter: {}", counter.get_value()); // Output: Counter: 2
    let _x = counter.value.borrow_mut();
    let _y = counter.value.borrow();
}


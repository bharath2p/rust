/*
 * 1. Cell provides interior mutability, but during compile time using .get/.set and .replace
 *    methods
 * 2. Allows you to mutate even when the data itself is behind the immutable reference.
 *    Example:
 *    Struct Counter is immutable, As per external mutablity (aka inheritered 
 *    mutability, variables in the structure is also immutable.
 *    However, only the count can be made mutable using Cell.
 * 3. Thread safety is not available. So used only in Single threaded application.
 * 4. Value inside Cell must implementy Copy (and Clone).
 */

use std::cell::Cell;

struct Counter {
    some_var: u32,
    count: Cell<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            some_var: 10,
            count: Cell::new(0),
        }
    }

    fn increment(&self) {
        let current = self.count.get();
        self.count.set(current + 1);
        /* Below code will throw error, unless self is made &mut, and in
         * the main counter is made mutable
         */
        //self.some_var+=1;
    }

    fn get_count(&self) -> i32 {
        self.count.get()
    }
}

fn main() {
    let counter = Counter::new();
    
    counter.increment();
    counter.increment();

    println!("Count: {}", counter.get_count()); // Output: Count: 2
    println!("some_var = {}", counter.some_var);
}

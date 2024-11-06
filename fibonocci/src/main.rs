fn fibonacci_for_loop(n: u32) -> () {
    let mut first = 0;
    let mut second = 1;
    let mut iter = first + second;
    let _i = 0;
    println!("{}", first);
    println!("{}", second);
    for _i in 3..=n {
        println!("{}", iter);
        first = second;
        second = iter;
        iter = first + second;
    }

}

fn fibonacci_recursion(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci_recursion (n-1) + fibonacci_recursion (n-2)
    }
}

fn main() {
    println!("Fibonacci for first 5 using for loop");
    fibonacci_for_loop (5);

    println! ("Fibonacci for first 5 using recursion");
    for i in 0..5 {
        println! ("{}", fibonacci_recursion (i));
    }
}

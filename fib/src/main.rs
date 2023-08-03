fn main() {
    // a fibionacci sequence is a list of numbers where the next number
    // is the sum of the previous two.

    let digits_of_fib = 30;

    print!("0 1 ");
    fib(0, 1, digits_of_fib - 2);

    println!("");
}

fn fib(previous: i64, current: i64, remaining: i64) {
    if remaining > 0 {
        let next = previous + current;
        print!("{} ", next);
        fib(current, next, remaining - 1);
    }
}

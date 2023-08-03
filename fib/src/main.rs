fn main() {
    // a fibionacci sequence is a list of numbers where the next number
    // is the sum of the previous two.

    let digits_of_fib = 30;

    let mut i = 0;
    let mut previous = 0;
    let mut current = 0;

    while i < digits_of_fib {
        if i == 0 {
            current = 0;
            previous = 0;
        } else if i == 1 {
            current = 1; 
            previous = 0;
        } else {
            let swap = previous;
            previous = current;
            current = swap + previous;
        }

        print!("{} ", current);

        i = i + 1;
    }

    println!("");
}

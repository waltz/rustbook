fn main() {
    let mut x = 5;
    println!("The current value of x is: {}", x);

    x = 6;
    println!("The current value of x is: {}", x);

    let x = x * 3;
    println!("The shadowed x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("biggest point cloud is: {}", MAX_POINTS);

    let spaces = "     ";
    let spaces = &mut spaces.len().to_string();
    println!("mutable spaces is: {}", spaces);
}

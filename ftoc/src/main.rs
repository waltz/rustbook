use std::io;

fn main() {
    println!("Please enter a temperature to convert to celsius:");
    let mut original_temp = String::new();

    io::stdin().read_line(&mut original_temp).expect("whoops, there was a problem reading your input!");

    let original_temp: i32 = original_temp.trim().parse().expect("sorry, could not parse your temp");
    println!("temp in fahrenheit: {}", original_temp);

    let temp_in_celsius: f64 = ((original_temp - 30) / 2).into();
    println!("temp in celsius: {}", temp_in_celsius);
}

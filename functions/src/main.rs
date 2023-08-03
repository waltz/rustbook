fn main() {
    another_doer(5, 24);

    let condition = true;
    let aeroplane = if condition {
        8
    } else {
        "peonies"
    };
    println!("aeroplane let'd expression: {}", aeroplane);
}

fn another_doer(x: u8, y: usize) {
    println!("white noise and cozy space, {}, but why though: {}", x, y);

    let z = {
        let x = 3;
        x + 1
    };
    println!("expr-ing: {}", z);


    let shaka = five();
    println!("shorrrk: {}", shaka);
}

fn five() -> i32 {
    5
}

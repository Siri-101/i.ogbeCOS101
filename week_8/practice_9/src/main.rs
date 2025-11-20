fn main() {
    let x:(i32, bool, f64) = (110, true, 10.9);
    print(x);
}

fn print(i:(i32, bool, f64)) {
    println!("inside print method");
    println!("{:?}",i);
}

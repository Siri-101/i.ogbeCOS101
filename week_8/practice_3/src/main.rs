fn main() {
    let v = vec!['R','U','S','T','A','C','I','A','N'];

    let mut input_1 = String::new();

    println!("Enter an index value from 0 to 8");

    std::io::stdin().read_line(&mut input_1).expect("Failed to read input");
    let index:usize = input_1.trim().parse().expect("Not a valid positive integer");

    let ch: Option<&char> = v.get(index);

    value(ch);
}
fn value (n: Option<&char>) {
    println!("Element of Vector: {:?}", n);
}
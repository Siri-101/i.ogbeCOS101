fn main() {
    let v = vec!['C','O','M','P','U','T','E','R'];

    let mut input_1 = String::new();

    println!("Enter an index value from 0 to 7");

    std::io::stdin().read_line(&mut input_1).expect("Failed to read input");
    let index:usize = input_1.trim().parse().expect("Not a valid positive integer");
    if index < 8 {
        let ch: char = v[index];
        println!("{} is the character for index[{}]", ch, index);
    }
    else {
        println!("Number must be less than or equal to 7");
    }
}

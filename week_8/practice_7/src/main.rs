fn main() {
    let data_type_tuple: (&str, f32, u8) = ("Rust", 3.8, 100);
    println!("Tuple contents {:?}", data_type_tuple);

    let no_data_type_tuple = ("Rust", true, 100);
    println!("Tuple contents {:?}", no_data_type_tuple);

    println!("Value at index - 0: {}", data_type_tuple.0);

    println!("Value at index - 1: {}", data_type_tuple.1);

    println!("Value at index - 2: {}", data_type_tuple.2);
}

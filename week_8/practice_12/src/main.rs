fn main() {
    let mut colors = ["red", "green", "blue", "orange"];
    println!("Original array: {:?}", colors);

    let sliced_colors = &mut colors[1..3];
    println!("from second to 4th item of array: {:?}", sliced_colors);

    sliced_colors[1] = "Purple";

    println!("Changed array: {:?}", sliced_colors);
}

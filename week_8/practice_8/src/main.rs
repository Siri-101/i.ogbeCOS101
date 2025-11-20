fn main() {
    let mut mountain_heights = ("Everest", 8848, "Fishtail", 6993);
    println!("Original Tuple contents {:?}", mountain_heights);

    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8516;

    println!("Changed Tuple contents {:?}", mountain_heights);
}

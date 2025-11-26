use std::fs;

fn main() {
    let mut file = std::fs::File::create("data.txt").unwrap();

    fs::remove_file("data.txt").expect("Couldnt remove file");
    println!("File is removed");
}

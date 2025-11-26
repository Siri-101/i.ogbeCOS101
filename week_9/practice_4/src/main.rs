use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    std::fs::File::create("data.txt").unwrap();

    let mut file_1 = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
    file_1.write_all("\n Hello class".as_bytes()).expect("Failed to edit");
    file_1.write_all("This is the appendage".as_bytes()).expect("Failed to append");
    println!("File append successful");

}

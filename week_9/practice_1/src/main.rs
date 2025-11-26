use std::io::Write;


fn main() {
    let announce = "Week 9 - Rust File Input & Output";
    let dept = "Department of Computer science";

    let mut file = std::fs::File::create("data.txt").unwrap();
    file.write_all("Welcome to rust programming language \n".as_bytes()).unwrap();
    file.write_all(announce.as_bytes()).unwrap();
    file.write_all(dept.as_bytes()).unwrap();

    println!("File created successfully");
}

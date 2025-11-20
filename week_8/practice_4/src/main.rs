fn main() {
    let name = vec!["David", "Jack", "Cano", "Kareem", "Salama", "Jacinta"];

    let age = vec![16,17,18,19,20,15];

    for i in 0..age.len() {
        println!("{} is {} years old", name[i], age[i]);
    }

}

fn main() {
    let mut city : Vec<String> = Vec::new();

    println!("This city vector has {} element", city.len());

    let mut input_1 = String::new();

    println!("How many cities do you want to enter");

    std::io::stdin().read_line(&mut input_1).expect("Failed to read input");
    let city_num:usize = input_1.trim().parse().expect("Not a valid positive integer");

    for count in 0..city_num {
        let mut input_2 = String::new();
        println!("Enter City {}", count+1);
        std::io::stdin().read_line(&mut input_2).expect("Failed to read input");
        let new_city = input_2.trim().parse().expect("Not a valid string");
        city.push(new_city);
    }

    println!("Your preferred cities are: \n");
    let mut count = 1;
    for i in city {
        println!("{} {}", count, i);
        count += 1;
    }
}

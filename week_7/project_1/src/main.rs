use std::io;

fn main() {
    println!("Welcome to our Area and Volume calculator \n you can choose from the option which shape you want the area or volume of:");
    println!(" 1. Area of Trapezium \n 2. Area of Rhombus \n 3. Area of Parallelogram \n 4. Area of Cube \n 5. Volume of Cylinder \n Type the number before the area or volume you want to find");


    let mut input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("Failed to read input");
    let index:usize = input_1.trim().parse().expect("not a valid positive integer");
    let formula_vec = vec![trapezium_area, rhombus_area, parallelogram_area, cube_area, cylinder_vol];
    if index > 5 {
        println!("Number not in the list, option is from 1 - 5");
    }
    else {
        let answer = formula_vec[index-1]();
        println!("The answer is: {}", answer)
    }
}


fn trapezium_area() -> f32{
    println!("Enter height of trapezium: ");
    let mut input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("Failed to read input");
    let height:f32 = input_1.trim().parse().expect("not a valid positive integer");

    println!("Enter base 1 of trapezium: ");
    let mut input_2 = String::new();
    io::stdin().read_line(&mut input_2).expect("Failed to read input");
    let base_1:f32 = input_2.trim().parse().expect("not a valid positive integer");

    println!("Enter base 2 of trapezium: ");
    let mut input_3 = String::new();
    io::stdin().read_line(&mut input_3).expect("Failed to read input");
    let base_2:f32 = input_3.trim().parse().expect("not a valid positive integer");

    let area = (height / 2.0) * (base_1 + base_2);
    return area;
}

fn rhombus_area () -> f32{
    println!("Enter first diagonal of rhombus: ");
    let mut input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("Failed to read input");
    let diagonal_1:f32 = input_1.trim().parse().expect("not a valid positive integer");

    println!("Enter 2nd Diagonal of Rhombus: ");
    let mut input_2 = String::new();
    io::stdin().read_line(&mut input_2).expect("Failed to read input");
    let diagonal_2:f32 = input_2.trim().parse().expect("not a valid positive integer");

    let area = (diagonal_1/2.0) * diagonal_2;
    return area;
}

fn parallelogram_area () -> f32{
    println!("Enter base of Parallelogram: ");
    let mut input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("Failed to read input");
    let base:f32 = input_1.trim().parse().expect("not a valid positive integer");

    println!("Enter altitude of Parallelogram: ");
    let mut input_2 = String::new();
    io::stdin().read_line(&mut input_2).expect("Failed to read input");
    let altitude:f32 = input_2.trim().parse().expect("not a valid positive integer");
    let area = base * altitude;
    return area;
}

fn cube_area () -> f32{
    println!("Enter length of side of cube: ");
    let mut input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("Failed to read input");
    let length:f32 = input_1.trim().parse().expect("not a valid positive integer");
    let area = 6.0 * length * length;
    return area;
}

fn cylinder_vol () -> f32{
    println!("Enter height of cylinder: ");
    let mut input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("Failed to read input");
    let height:f32 = input_1.trim().parse().expect("not a valid positive integer");

    println!("Enter radius of cylinder: ");
    let mut input_2 = String::new();
    io::stdin().read_line(&mut input_2).expect("Failed to read input");
    let radius:f32 = input_2.trim().parse().expect("not a valid positive integer");
    let volume = (22.0/7.0) * radius * radius * height;
    return volume;
}
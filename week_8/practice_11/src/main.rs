fn main() {
    let numbers = [1,2,3,4,5];
    println!("Original array: {:?}", numbers);

    let slice1 = &numbers[1..3];
    println!("Second and third element of array: {:?}", slice1);

    let slice2 = &numbers[..3];
    println!("first to third element of list: {:?}", slice2);

    let slice3 = &numbers[2..];
    println!("from third to last element of array: {:?}", slice3);

    let slice4 = &numbers[..];
    println!("Prints all elements of list: {:?}", slice4);

}

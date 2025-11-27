use std::fs;
use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("data.csv").unwrap();
    file.write_all(" , , PAU SMIS \n ".as_bytes()).expect("Failed to write file");
    file.write_all("Student Name, Matric. Number, Department, Level \n".as_bytes()).expect("Failed to write file");

    let name = vec!["Oluchi Mordi,", "Adams Aliyu,", "Shania Bolade,", "Adekunle Gold,", "Blanca Edemoh,"];
    let matric_num = vec!["ACC10211111,", "ECO10110101,", "CSC10328828,", "EEE11020202,", "MEE10202001,"];
    let departmnent = vec!["Accounting,", "Economics,", "Computer,", "Electrical,", "Mechanical,"];
    let level = vec!["300,", "100,", "200,", "200,", "100,"];

    // file.write_all("Lager,
    //     33 Export,
    //     Desperados,
    //     Goldberg,
    //     Gulder,
    //     Heineken,
    //     Star".as_bytes()).expect("Failed to edit");

    // file.write_all("Stout,
    //     Legend,
    //     Turbo King,
    //     Williams".as_bytes()).expect("Failed to edit");
    let mut index = 0;
    for i in &name {
        file.write_all((name[index]).as_bytes()).expect("Failed to write file");
        file.write_all((matric_num[index]).as_bytes()).expect("Failed to write file");
        file.write_all((departmnent[index]).as_bytes()).expect("Failed to write file");
        file.write_all((level[index]).as_bytes()).expect("Failed to write file");
        file.write_all("\n".as_bytes()).expect("Failed to write file");
        index += 1;
    }

    // file.write_all(name.as_bytes())
    //     .expect("Failed to write file");
    
    
}

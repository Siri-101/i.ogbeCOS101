use std::fs;
use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("data.csv").unwrap();
    file.write_all("S/N, NAME OF COMMISIONER, Ministry, Geopolitical Zone \n".as_bytes()).expect("Failed to write file");

    let name = vec![",Aigbogun Alamba Daudu,", ",Murtala Afeez Bendu,", ",Okorocha Calistus Ogbona,", ",Adewale Jimoh Akanbi,", ",Osazuwa Faith Etieye,", ];
    let ministry = vec!["Internal Affairs,", "Justice,", "Defense,", "Power & Steel,", "Petroleum,",];
    let departmnent = vec!["South West,", "North East,", "South South,","South West,", "South East,",];

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
        file.write_all((index+1).to_string().as_bytes()).expect("Failed to write file");
        file.write_all((name[index]).as_bytes()).expect("Failed to write file");
        file.write_all((ministry[index]).as_bytes()).expect("Failed to write file");
        file.write_all((departmnent[index]).as_bytes()).expect("Failed to write file");
        file.write_all("\n".as_bytes()).expect("Failed to write file");
        index += 1;
    }

    // file.write_all(name.as_bytes())
    //     .expect("Failed to write file");
    
    
}

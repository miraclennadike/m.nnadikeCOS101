use std::fs::File;
use std::io::Write;

fn main() {
    // Dataset 1: Names of Commissioners
    let names = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiye",
    ];

    // Dataset 2: Ministries
    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    // Dataset 3: Geopolitical Zones
    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Display header
    println!("EFCC MERGED DATA");
    println!("---------------------------------------------------------------");
    println!("S/N | Name | Ministry | Geopolitical Zone");
    println!("---------------------------------------------------------------");

    // Create output file
    let mut file = File::create("efcc_merged_data.txt")
        .expect("Unable to create file");

    // Write header to file
    writeln!(file, "EFCC MERGED DATA").unwrap();
    writeln!(file, "S/N | Name | Ministry | Geopolitical Zone").unwrap();
    writeln!(file, "---------------------------------------------------------------").unwrap();

    // Merge datasets using index
    for i in 0..names.len() {
        println!(
            "{} | {} | {} | {}",
            i + 1,
            names[i],
            ministries[i],
            zones[i]
        );

        writeln!(
            file,
            "{} | {} | {} | {}",
            i + 1,
            names[i],
            ministries[i],
            zones[i]
        )
        .unwrap();
    }

    println!("\nMerged data saved successfully!");
}


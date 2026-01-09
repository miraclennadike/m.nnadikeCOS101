use std::fs::File;
use std::io::Write;

fn main() {
    // Create a file
    let mut file = File::create("nigerian_breweries.txt")
        .expect("Could not create file");

    // Content to be written into the file
    let content = 
"LAGER:
- 33 Export
- Desperados
- Goldberg
- Gulder
- Heineken
- Star

STOUT:
- Legend
- Turbo King
- Williams

NON-ALCOHOLIC:
- Maltina
- Amstel Malta
- Malta Gold
- Fayrouz
";

    // Write content to the file
    file.write_all(content.as_bytes())
        .expect("Could not write to file");

    println!("File created and data saved successfully!");
}


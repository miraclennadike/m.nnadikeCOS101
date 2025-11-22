use std::io;

// Define a compound data type (struct) to hold each candidate's information
struct Developer {
    name: String,
    years_of_experience: u32,
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    println!("=== EY Nigeria: Developer Experience Checker ===");

    // Ask how many candidates will be interviewed
    let num = read_input("Enter the number of developers to check:")
                .parse::<usize>()
                .expect("Please enter a valid number");

    let mut developers: Vec<Developer> = Vec::new();

    for i in 1..=num {
        println!("\nCandidate {}:", i);

        let name = read_input("Enter candidate's name:");

        let years = read_input("Enter candidate's years of programming experience:")
                        .parse::<u32>()
                        .expect("Please enter a valid number");

        developers.push(Developer {
            name,
            years_of_experience: years,
        });
    }

    // Find the developer with the highest experience
    let mut most_experienced = &developers[0];
    for dev in &developers {
        if dev.years_of_experience > most_experienced.years_of_experience {
            most_experienced = dev;
        }
    }

    println!("\n=== RESULT ===");
    println!(
        "The most experienced developer is: {} with {} years of programming experience!",
        most_experienced.name, most_experienced.years_of_experience
    );
}


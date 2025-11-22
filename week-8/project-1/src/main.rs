use std::io;

fn main() {
    println!("=== APS Level Checker for Federal Government of Nigeria ===");

    // Vectors representing each job level
    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    // Vector for APS levels
    let aps_levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    // Ask for profession
    let mut profession = String::new();
    println!("Enter profession (Office Admin / Academic / Lawyer / Teacher):");
    io::stdin().read_line(&mut profession).expect("Failed input");
    let profession = profession.trim().to_lowercase();

    // Ask for job title
    let mut title = String::new();
    println!("Enter job title exactly as in the table:");
    io::stdin().read_line(&mut title).expect("Failed input");
    let title = title.trim();

    // Function to search for role inside a vector
    fn find_level(role: &str, list: &Vec<&str>) -> Option<usize> {
        for (i, &val) in list.iter().enumerate() {
            if val.to_lowercase() == role.to_lowercase() {
                return Some(i);
            }
        }
        None
    }

    // Determine APS Level based on profession selected
    let result = match profession.as_str() {
        "office admin" => find_level(title, &office_admin),
        "academic" => find_level(title, &academic),
        "lawyer" => find_level(title, &lawyer),
        "teacher" => find_level(title, &teacher),
        _ => None,
    };

    // Display result
    match result {
        Some(idx) => println!("\n{} belongs to level: {}", title, aps_levels[idx]),
        None => println!("\nJob title not found. Please check your input."),
    }
}


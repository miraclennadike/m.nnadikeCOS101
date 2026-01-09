use std::fs::File;
use std::io::Write;

// Define a structure for a student
struct Student {
    name: String,
    matric_no: String,
    department: String,
    level: u32,
}

fn main() {
    // Create a vector of students
    let students = vec![
        Student {
            name: String::from("Oluchi Mordi"),
            matric_no: String::from("ACC10211111"),
            department: String::from("Accounting"),
            level: 300,
        },
        Student {
            name: String::from("Adams Aliyu"),
            matric_no: String::from("ECO10110101"),
            department: String::from("Economics"),
            level: 100,
        },
        Student {
            name: String::from("Shania Bolade"),
            matric_no: String::from("CSC10328828"),
            department: String::from("Computer"),
            level: 200,
        },
        Student {
            name: String::from("Adekunle Gold"),
            matric_no: String::from("EEE11020202"),
            department: String::from("Electrical"),
            level: 200,
        },
        Student {
            name: String::from("Blanca Edemoh"),
            matric_no: String::from("MEE10202001"),
            department: String::from("Mechanical"),
            level: 100,
        },
    ];

    // Display student details
    println!("PAU SMIS");
    println!("-------------------------------------------------");
    println!("Name | Matric No | Department | Level");
    println!("-------------------------------------------------");

    for student in &students {
        println!(
            "{} | {} | {} | {}",
            student.name,
            student.matric_no,
            student.department,
            student.level
        );
    }

    // Create a file
    let mut file = File::create("pau_smis.txt")
        .expect("Could not create file");

    // Write header to file
    writeln!(file, "PAU SMIS").unwrap();
    writeln!(file, "Student Name | Matric Number | Department | Level").unwrap();
    writeln!(file, "-------------------------------------------------").unwrap();

    // Write student data to file
    for student in &students {
        writeln!(
            file,
            "{} | {} | {} | {}",
            student.name,
            student.matric_no,
            student.department,
            student.level
        )
        .unwrap();
    }

    println!("\nStudent records saved successfully to file!");
}


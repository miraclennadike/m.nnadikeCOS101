use std::io;

// function to read user input and convert to f64
fn read_value(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse::<f64>().expect("Please enter a number")
}

// Trapezium: (height/2) * (base1 + base2)
fn area_trapezium() {
    let height = read_value("Enter the height:");
    let base1 = read_value("Enter base 1:");
    let base2 = read_value("Enter base 2:");
    let area = (height / 2.0) * (base1 + base2);
    println!("Area of Trapezium = {}", area);
}

// Rhombus: 1/2 × diagonal1 × diagonal2
fn area_rhombus() {
    let d1 = read_value("Enter diagonal 1:");
    let d2 = read_value("Enter diagonal 2:");
    let area = 0.5 * d1 * d2;
    println!("Area of Rhombus = {}", area);
}

// Parallelogram: base × altitude
fn area_parallelogram() {
    let base = read_value("Enter base:");
    let altitude = read_value("Enter altitude:");
    let area = base * altitude;
    println!("Area of Parallelogram = {}", area);
}

// Cube: 6 × (side)²
fn area_cube() {
    let side = read_value("Enter length of one side:");
    let area = 6.0 * side * side;
    println!("Area of Cube = {}", area);
}

// Cylinder Volume: π × radius² × height
fn volume_cylinder() {
    let radius = read_value("Enter radius:");
    let height = read_value("Enter height:");
    let volume = std::f64::consts::PI * radius * radius * height;
    println!("Volume of Cylinder = {}", volume);
}

fn main() {
    println!("Select the calculation you want to perform:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: i32 = choice.trim().parse().expect("Please enter a number");

    match choice {
        1 => area_trapezium(),
        2 => area_rhombus(),
        3 => area_parallelogram(),
        4 => area_cube(),
        5 => volume_cylinder(),
        _ => println!("Invalid choice!"),
    }
}


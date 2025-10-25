use std::io;

fn main() {
    let mut experience = String::new();
    println!("are you experienced? if you are, type 'experienced'If you aren't, type 'inexperienced'");
    io::stdin().read_line(&mut experience).expect("Failed to read line");
    let experience = experience.trim();

    let age: u8;
    loop {
        let mut age_input = String::new();
        println!("Enter your age:");
        io::stdin().read_line(&mut age_input).expect("Failed to read line");

        match age_input.trim().parse::<u8>() {
            Ok(num) => {
                age = num;
                break;
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }

    if experience == "experienced" && age >= 40 {
        println!("your incentive is N1,560,000 per month")
    }
    else if experience == "exprienced" && age > 30 && age < 40 {
        println!("Your incentive is N1,480,000 per month")
    }
    else if experience == "experienced" && age < 28 {
        println!("Your incentive is N1,300,000 per month")
    }
    else {
        println!("Your incentive is N100,000 per month")
    }



}
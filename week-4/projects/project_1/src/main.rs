use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Please enter the coefficent a:");
    io::stdin().read_line(&mut a).expect("Failed to read line");
    let a: f64 = a.trim().parse().expect("Please enter a valid number");

    println!("Please enter the coefficent b:");
    io::stdin().read_line(&mut b).expect("Failed to read line");
    let b: f64 = b.trim().parse().expect("Please enter a valid number");

    println!("Please enter the coefficent c:");
    io::stdin().read_line(&mut c).expect("Failed to read line");
    let c: f64 = c.trim().parse().expect("Please enter a valid number");

    println!("The quadratic equation is: {}x² + {}x + {} = 0", a, b, c);

    let discriminant = (b * b) - 4.0 * a * c;
    println!("The discriminant is {}", discriminant);

    if discriminant > 0.0 {
        let x1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let x2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Your roots are {} and {}", x1, x2);
    }
    else if discriminant == 0.0{
        let x = (-b) / (2.0 * a);
        println!("x = {}",x);
    }
    else {
        let real_part =(-b) / (2.0 * a);
        let imaginary_part = (-discriminant).sqrt() / (2.0 * a);

        println!("x1 = {} + {}i", real_part, imaginary_part);
        println!("x2 = {} + {}i", real_part, imaginary_part);
    }

}

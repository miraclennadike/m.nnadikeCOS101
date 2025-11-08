use std::io;

fn main() {
    let p = "Poundo Yam/Edinkaiko Soup";
    let f = "Fried Rice & Chicken";
    let a = "Amala & Ewedu Soup";
    let e = "Eba & Egusi Soup";
    let w = "White Rice & Stew";

    let p_price = 3_200.0;
    let f_price = 3_000.0;
    let a_price = 2_500.0;
    let e_price = 2_000.0;
    let w_price = 2_500.0;

    println!("The menu for today is");
    println!("1. {} - {} (Type 'p')", p, p_price);
    println!("2. {} - {} (Type 'f')", f, f_price);
    println!("3. {} - {} (Type 'a')", a, a_price);
    println!("4. {} - {} (Type 'e')", e, e_price);
    println!("5. {} - {} (Type 'w')", w, w_price);

    let mut choice = String::new();
    println!("Type the letter corresponding to your choice of meal");
    io::stdin().read_line(&mut choice).expect("failed to read line");
    let choice = choice.trim().to_lowercase();

    let mut quantity = String::new();
    println!("Enter your quantity");
    io::stdin().read_line(&mut quantity).expect("failed to read line");
    let quantity:f64 = quantity.trim().parse().expect("Not a valid number");

    let total:f64 = match choice.as_str() {
        "p" => (p_price * quantity).into(),
        "f" => (f_price * quantity).into(),
        "a" => (a_price * quantity).into(),
        "e" => (e_price * quantity).into(),
        "w" => (w_price * quantity).into(),
         _ => {
            println!("Invalid choice ");
            return;
          }
    };
    
    println!("Your total is: {}", total);

    if total > 10_000.0 {
        println!("you will receive a 5% discount");
        let discount:f64 = 0.05 * (total);
        let total_after_discount = total - discount;
        println!("Your total - discount is: {}", total_after_discount);
        return;
    };

    println!("Thank you coming");

    let mut repeat_choice =String::new();
    println!("would you like to place another order (yes/no)");
    io::stdin().read_line(&mut repeat_choice).expect("failed to read line");
    let repeat_choice = repeat_choice.trim().to_lowercase();

    if repeat_choice == "yes" {
        main();
    }
    else {
        println!("goodbye!");
    }   
}
// Define a struct for Laptop
struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    // Method to calculate total cost for a given quantity
    fn total_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    // Create laptop objects
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650_000,
    };

    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755_000,
    };

    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550_000,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850_000,
    };

    let quantity = 3;

    // Calculate total cost
    let total_cost =
        hp.total_cost(quantity) +
        ibm.total_cost(quantity) +
        toshiba.total_cost(quantity) +
        dell.total_cost(quantity);

    // Display results
    println!("Laptop Purchase Summary");
    println!("------------------------");
    println!("{} laptops from each brand\n", quantity);

    println!("HP cost: ₦{}", hp.total_cost(quantity));
    println!("IBM cost: ₦{}", ibm.total_cost(quantity));
    println!("Toshiba cost: ₦{}", toshiba.total_cost(quantity));
    println!("Dell cost: ₦{}", dell.total_cost(quantity));

    println!("\nTotal cost of purchase: ₦{}", total_cost);
}


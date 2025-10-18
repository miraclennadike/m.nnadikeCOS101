fn main() {
	let sales = [450_000.00, 450_000.00, 1_500_000.00, 750_000.00, 750_000.00, 750_000.00, 2_850_000.00, 2_850_000.00, 2_850_000.00, 250_000.00];

	// finding the total and average of the sales figures

	let total: f64 = sales.iter().sum();
	let average: f64 = total / sales.len() as f64;

	//  printing the results of the total and average sales figures

	println!("The total sales is: {}", total);
	println!("The average sales is: {}", average);
}
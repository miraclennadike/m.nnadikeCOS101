fn main() {
	let p:f64 = 510_000.00;
	let r:f64 = 5.00;
	let n:f64 = 3.00;

	// compound interest calculation
	let a = p * (1.0 - (r/100.0)).powf(n);
	println!("Amount is {}", a);
	let ci = p - a;
	println!("Compund Interest is {}", ci);
}
use std::io;

fn read_number(number: &str) -> i32 {

	let mut input = String::new();
	println!("{}",number);
	io::stdin().read_line(&mut input).expect("Fehler bei der Eingabe");
	input.trim().parse().expect("Keine gültige Eingabe")
}

fn main() {

	println!("Bitte gib zwei Zahlen ein, die wir addieren wollen.");
	let number1 = read_number("Die erste Zahl:");
	let number2 = read_number("Die zweite:");

	let result = number1 + number2;

	println!("Das Ergebnis ist: {}" , result);
}

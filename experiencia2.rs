use std::io;

fn main() {
	println!("**Guess the number**");
	println!("Please input your guess:");

	let mut input = String::new();
	//mut -> significa que a variavel pode mudar
	// sem mut significa que é estática

	//new() é um método da String que cria uma nova String vazia

	io::stdin().read_line(&mut input)
		.expect("failed");

	println!("You guessed: {}", input);


}
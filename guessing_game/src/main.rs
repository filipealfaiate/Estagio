use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main()
{
	println!("**Guess the number**");

	let secret_number = rand::thread_rng().gen_range(0, 101);

	println!("The secret number is: {}", secret_number);

	println!("Please input your guess:");

	let mut input = String::new();
	//mut -> significa que a variavel pode mudar
	// sem mut significa que é estática

	//new() é um método da String que cria uma nova String vazia

	let input: u32 = input.trim().parse().expect("Please type a number!");


	io::stdin().read_line(&mut input)
		.expect("failed");

	println!("You guessed: {}", input);

	match input.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

}

//Rust Enhanced
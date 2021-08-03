use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main()
{
	println!("**Guess the number**");

	let secret_number = rand::thread_rng().gen_range(0, 101);

	loop
	{
		println!("Please input your guess:");

		let mut input = String::new();
		//mut -> significa que a variavel pode mudar
		// sem mut significa que é estática
		//new() é um método da String que cria uma nova String vazia

		io::stdin().read_line(&mut input)
			.expect("failed");

		let input: u32 = match input.trim().parse()
							{ Ok(num) => num, Err(_) => continue,};

		println!("You guessed: {}", input);

		match input.cmp(&secret_number) {
	        Ordering::Less => println!("Too small!"),
	        Ordering::Greater => println!("Too big!"),
	        Ordering::Equal => {println!("You win!"); break;}
	    }
	 }
}
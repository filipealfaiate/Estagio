mod modulos;
use modulos::{eat_at_restaurant};
use std::fmt::Result;
use std::io::Result as IoResult;
use rand::Rng;

fn func1() -> Result
{
	Ok(())
}

fn func2() -> IoResult<()>
{
	Ok(())
}

fn main()
{
	eat_at_restaurant();

	let resultado =func1();
	let resultado1 = func2();
	println!("{:?} e {:?}", resultado, resultado1);

	//para adicionar pacotes externos, como o random, precisamos de adicionar no ficheiro Cargo.tml na parte das dependencias rand = "0.5.5"
	let secret_number = rand::thread_rng().gen_range(0, 101);

	println!("{:?}", secret_number);
}

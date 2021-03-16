fn main()
{
	//struct + enum
	let home = IpAddrKind
				{
					kind: IpAddrKindEnum::V4,
					address: String::from("127.0.0.1"),
				};

	//apenas com enum
	let loopback = IpAddrKindEnum::V6(String::from("::1"));

	let work = IpAddrKindEnum::V8(127, 0, 0, 1);

	println!("struct + enum:\n{:#?}", home);
	println!("\napenas com enum(string):\n{:?}", loopback);
	println!("\napenas com enum(int u8):\n{:?}", work);
	
	let w = Message::Write(String::from("halooo"));
	let m = Message::Move { x: 10, y: -10 };
	let c = Message::ChangeColor(255, 255, 255);
	let q = Message::Quit;
	w.call();
	m.call();
	c.call();
	q.call();

	println!("\n{:#?}", m);
	println!("\n{:#?}", w);
	println!("\n{:#?}", c);

	let _some_num = Some(5);
	let _some_str = Some("string");
	//let absent_num: Option<i32> = None;

	//let x: Option<i8> = Some(5);

	//let sum = some_num + x;

	let coin = Coin::Penny;

	println!("a penny: {}", value_in_cents(coin));
	println!("a nickel: {}", value_in_cents(Coin::Nickel));
	println!("a dime: {}", value_in_cents(Coin::Dime));
	println!("a quarter: {}", value_in_cents(Coin::Quarter(UsState::Alaska)));
	println!("another quarter: {}", value_in_cents(Coin::Quarter(UsState::Alabama)));

	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);

	println!("five: {:?}", five);
	println!("six: {:?}", six);
	println!("none: {:?}", none);

	let impar_ate7 = 1;//u8
	match impar_ate7
	{
		1 => println!("uno"),
		3 => println!("tree"),
		5 => println!("cinco"),
		7 => println!("siete"),
		_ => (), //vai dar match com todas as outras possibilidade que nao estao a cima representadas. os "()" dizem para nao fazer nada nestes caso
	};

	let some_u8_value = Some(3u8);
	if let Some(3) = some_u8_value			//funciona igual ao match
	{
		println!("three ;)");
	}

	let mut count = 0;
	let coin_if = Coin::Quarter(UsState::Alaska);
	let coin_match = Coin::Quarter(UsState::Alaska);

	match coin_match
	{
		Coin::Quarter(state) => println!("State quarter from {:?} match function", state),
		_ => count += 1,
	}
	//ou
	if let Coin::Quarter(state) = coin_if
	{
		println!("State quarter from {:?} if function", state);
	}
	else
	{
		count += 1;
	}
}

/*#[derive(Debug)]
enum Option <T>
{
	Some(T),
	None,
}*/

#[derive(Debug)]
enum Coin
{
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
}

#[derive(Debug)]
enum UsState
{
	Alabama,
	Alaska,
}

#[derive(Debug)]
enum IpAddrKindEnum
{
	V4,
	V6(String),
	V8(u8, u8, u8, u8),
}

#[derive(Debug)]
enum Message
{
	//as "variaveis" tem de ser em letra grande
	Quit,						//include no data
	Move{ x: i32, y: i32},		//include struct
	Write(String),				//include string
	ChangeColor(i32, i32, i32), //include int i32
	//este enum é equivalente a 4 structs diferentes
}

impl Message
{
	fn call(&self){}
}


#[derive(Debug)]
struct IpAddrKind
{
	kind: IpAddrKindEnum,
	address: String,
}

fn value_in_cents(coin: Coin) -> u8
{
	match coin
	{
		Coin::Penny =>  {
							println!("cada braco do match pode ter várias linhas de codigo, apesar de nao ser muito usual");
							1
						},
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
									println!("State quarte from {:?}!", state);
									25
								},
	} //este match tem 4 braços, mas podemos ter os que quisermos
}

fn plus_one(x: Option<i32>) -> Option<i32>
{
	match x
	{
		None => None,
		Some(i) => Some(i+1),
	}
}
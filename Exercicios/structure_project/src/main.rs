#[derive(Debug)]
struct User {
	name: String,
	email: String,
	pass: u32,
	active: bool,
}

#[derive(Debug)]
struct Rectangulo
{
	width: u32,
	height: u32,
}

impl Rectangulo {  //metodo da struct Rectangulo, qualquer metodo criado para esta struct será construido dentro do impl(implementation)
	fn area_s(& self) -> u32     //&self é o objecto que é passado, exemplo na main
	{
		self.width * self.height
	}

	fn can_hold(&self, x: &Rectangulo) -> bool
	{
		self.area_s() > x.area_s()
	}

	fn square(size: u32) -> Rectangulo
	{
		Rectangulo
		{
			width: size,
			height: size,
		}
	}
}

fn bulid_user(name: String, email: String) -> User
{
	User
	{
		name, //instead of name: name
		active: true,
		email, //instead of email:email
		pass: 1234,
	}
}

fn main() {
	let fil = bulid_user(String::from("Filipe"), String::from("123@gmail.com"));
	let user = User
				{
					name: String::from("Miguel"),
					email: String::from("321@gmail.com"),
					..fil
				};
	println!("nome: {}", fil.name);
	println!("email: {}", fil.email);
	println!("pass: {}", fil.pass);
	println!("active: {}", fil.active);
	println!("nome: {}", user.name);
	println!("email: {}", user.email);
	println!("pass: {}", user.pass);
	println!("active: {}", user.active);


	struct Color(i32, i32, i32);
	struct Coordenates(i32, i32, i32);

	let black = Color(0,0,0);
	let white = Color(255, 255, 255);
	let origin = Coordenates(0,0,0);

	println!("black: ({},{},{})", black.0, black.1, black.2);
	println!("white: ({},{},{})", white.0, white.1, white.2);
	println!("origin: ({},{},{})", origin.0, origin.1, origin.2);

	//area do retangulo com tuplos

	let rect = (30,50);
	println!("A area do rectangulo com tuplos: {}", area_t(rect));

	let rect1 = Rectangulo{width:30, height:50};
	let rect2 = Rectangulo{width:10, height:40};
	let rect3 = Rectangulo{width:60, height:45};
	let quadrado = Rectangulo::square(38);     //forma de chamar uma funcao que esta no tipo antes dos ::

	println!("Area do rectangulo com struct: {}", rect1.area_s()); //queremos emprestar o valor e nao dar a propriedade

	println!("\nEstrutura de rect1 com :?: \n{:?}", rect1);
	// aqui {:?} serve para fazer debug, mostrando o que está na struct no momento
	// para funcionar temos de colocar #[derive(Debug)] por cima da struct
	// podemos tambem usar {:#?} para mostrar a struct formatada, é mais util quando for uma struct maior
	println!("Estrutura de rect1 com :#?: \n{:#?}", rect1);

	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
	println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
	println!("as medidas do quadrado sao: {:?}", quadrado);
	println!("Can rect1 hold quadrado? {}", rect1.can_hold(&quadrado));


}

fn area_t(dimensoes: (u32, u32)) -> u32
{
	dimensoes.0 * dimensoes.1
}



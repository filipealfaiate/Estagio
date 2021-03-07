fn main() {
	//stack/pilha LIFO(last in first out)
	//heap:
	//		-1º: alocar espaço na memoria;
	//		-2º: returna um pointer do inicio desse espaço;

	let s1 = String::from("Filipe"); //quando chamamos String::from estamos a alocar espaço na memoria para o que está ()

	//s1.push_str(" Alfaiate"); //concatenação de strings

	println!("{:?}", s1);

	let s2 = s1 + &" Alfaiate".to_string(); //tambem pode ser uma concatnação

	println!("{:?}", s2); //nao posso chamar s1 porque s1 foi copiado para s2 e em seguida "apagado"

	let s3 = s2.clone() + &" you'r the best";

	println!("Now we can use boths var becouse we clone: s2 = {} and s3 = {}", s2, s3);
	//porque com {:?} põe "" na string e {} sem ""? porque nos inteiros fica igual em ambas as formas?


	//tipos de variaveis que implementam copy:
	//			- todos os int;
	//			- bool;
	//			- todos os float;
	//			- char;
	//			- qualquer tuplo que contenha tipos de variaveis que tenham o copy;

	let s = String::from("Hello");

	println!("Befor function: {}", s);
	ownership(s);
	//println!("After function: {}", s); //esta linha nao pode ser executada porque foi copiado para dentro da função
										 //mas se fizer s.clone() poderei continuar pois a variavel foi clonada e nao copiada

	let x = 5;

	println!("Befor function: {:?}", x);
	copia(x);
	println!("After function: {}", x);


	let exp1 = gives_ownership();

	println!("{}", exp1);

	let exp2 = String::from(" por hoy");

	let exp3 = takes_and_gives_back(exp2);

	println!("{}", exp1 + &exp3);
	//println!("{}", exp2); // nao posso usar porque o valor foi copiado

	let mut loucura = String::from("what?");

	loucura = possible(loucura);

	println!("{}", loucura + &" nice ;)");

	//colocar a variavel mut dá para rescreve-la mesmo quando é copiada e "perdida"

}

fn possible(y: String) -> String
{
	let x = String::from("is it possi... ");
	x + &y
}

fn takes_and_gives_back(x: String) -> String
{
	x
}

fn gives_ownership() -> String
{
	let x = String::from("Finito");
	x
}

fn ownership(x: String)
{
	println!("During function: {}", x);
}

fn copia(x: i32)
{
	println!("During function: {}", x);
}

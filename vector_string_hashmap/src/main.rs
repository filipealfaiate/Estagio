//mod funcoes;
//use funcoes::{func};
use std::collections::HashMap;

fn main()
{
	//craiar um array vazio
	let mut arr: Vec<i32> = Vec::new();
	let mut v = vec![1, 2, 3];
	//let mut arr_vec: Vec<Vec<i32>> = Vec::new();

	arr.push(5);
	arr.push(6);
	arr.push(7);

	println!("{}, {}, {}, {:?}, {:?}", v[0], v[1], v[2], v, arr);

	/*arr_vec.push(v);
	arr_vec.push(arr);

	println!("{:?}", arr_vec);*/

	let third: &i32 = &v[2]; //devolve uma referencia
	println!("este e o terceiro numero: {}", third);

	match v.get(2) //devolve um Option<&T>
	{
		Some(third) => println!("o terceiro elemento: {}", third),
		None => println!("nao existe"),
	}

	match v.get(100) //devolve um Option<&T>
	{
		Some(third) => println!("o terceiro elemento: {}", third),
		None => println!("nao existe"),
	}

	print!("[");
	for elemento in &v 					//usamos o pointer para emprestar o valor
	{
		if elemento == &v[v.len()-1]
		{
			print!("{}]", elemento);
		}

		else
		{
			print!("{}, ", elemento);
		}

	}
	println!();

	for elemento in &mut v
	{
		*elemento += 1;			//para somar o valor temos que desreferencia-lo
	}
	println!("{:?}", v);


	//arrays com varios tipos
	let row = vec!
				[
					SpreadsheetCell::Int(3),
					SpreadsheetCell::Text(String::from("cell")),
					SpreadsheetCell::Float(4.20),
				];
	println!("{:?}", row);

	match row.get(0)
	{
		Some(elemento) => println!("{:?}", elemento),
		None =>(),
	}


	//Strings

	//diferentes maneiras de meter coisas na string

	//declarar uma vareavel String vazia
	let mut s = String::new();
	s = s + "data";					//concatnacao de strings
	s.push_str(" push1");			//concatnacao de strings

	let data = "data 0";
	let s0 = data.to_string();

	let s1 = "data 1".to_string();

	let s2 = String::from("data 2");

	//em vez de usar varias concatnacoes podemos usar o format!(), ainda tendo a vantagem de que nao perde o ownership
	let s3 = format!("s3: {} e {}", s1, s2); // == let s3 = "s3: " + s1 + " e " + &s2;

	println!("s: {}, s0: {}, s1: {}, s2: {}, {}", s, s0, s1, s2, s3);


	//indexar strings

	let hello = "Hello World!".to_string();

	println!("{}", &hello[0..5]);

	for elemento in hello.chars()
	{
		print!("{}", elemento);
	}
	println!(); 

	for elemento in hello.bytes()
	{
		print!("{} ", elemento);
	}
	println!();


	//hash map
	let mut scores = HashMap::new();    //cria uma hash

	//primeiro argumento é o tipo para calcular a posicao e o segundo argumento é o que queremos guardar
	scores.insert("Benfica".to_string(), 10);
	scores.insert("Porto".to_string(), 5);	
	scores.insert("Sporting".to_string(), 2);

	let teams = vec![String::from("Braga"), String::from("Famalicao")];
    let initial_scores = vec![7, 1];
	let pontos: HashMap<_,_> = teams.into_iter().zip(initial_scores.into_iter()).collect();
	// os underscore definem um tipo qualquer, neste caso vai ser um String e um i32 porque o vec teams é String e o vecinitial_scores é um i32

	println!("{:?}, {:?}", scores, pontos);

	let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    println!("{:?}", map,);

    
    let equipa = String::from("Benfica");
    println!("{:?}", scores.get(&equipa).unwrap());

    for (key, value) in &scores
    {
    	println!("{} {}", key, value);
    }

    //neste caso reescreve o valor
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    //neste caso so insere o valor se a chave nao existir
    scores.entry(String::from("Blue")).or_insert(50);

    //atualizar o valor com base no antigo
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace()
    {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    

}

#[derive(Debug)]
enum SpreadsheetCell
{
	Int(i32),
	Float(f64),
	Text(String),
}
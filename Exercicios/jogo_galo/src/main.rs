use std::io;

fn main()
{
	loop {
		println!("\n0 - Novo Jogo \n1 - Sair");

		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("failed");

		let input: usize = match input.trim().parse()
							{ 
								Ok(num) => num,
								Err(_) => continue,
							};
		if input == 1 { break; }
		if input > 1 {println!("Insira 0 ou 1");}
		else
		{
			let mut i = 0;

			let mut arr=[
							Tabuleiro::Coordenadas(0),
							Tabuleiro::Linha("|".to_string()),
							Tabuleiro::Coordenadas(1),
							Tabuleiro::Linha("|".to_string()),
							Tabuleiro::Coordenadas(2),
							Tabuleiro::Linha("|".to_string()),
							Tabuleiro::Coordenadas(3),
							Tabuleiro::Linha("|".to_string()),
							Tabuleiro::Coordenadas(4),
							Tabuleiro::Linha("|".to_string()),
							Tabuleiro::Coordenadas(5),
							Tabuleiro::Linha("|".to_string()),
							Tabuleiro::Coordenadas(6),
							Tabuleiro::Linha("|".to_string()),
							Tabuleiro::Coordenadas(7),
							Tabuleiro::Linha("|".to_string()),
							Tabuleiro::Coordenadas(8),
							Tabuleiro::Linha("|".to_string()),
						];

			println!();
			let mut j = 0;

			for elemento in &arr
			{
				match elemento
				{
					Tabuleiro::Coordenadas(coordenadas) => print!("{}", coordenadas),
					Tabuleiro::Linha(linha) => print!("{}", linha),
					Tabuleiro::Jogada(jogada) => print!("{}", jogada),
				};

				if j == 5 || j == 11 || j == 17
				{
					println!();
				}

				j += 1;
			}

			while i<9
			{
				let mut input = String::new();
				io::stdin().read_line(&mut input).expect("failed");

				let input: usize = match input.trim().parse()
									{ 
										Ok(num) => num,
										Err(_) => continue,
									};

				if input < 9
				{
					if i%2 == 0
					{
						match &arr[input*2]
						{
							Tabuleiro::Jogada(x) =>	{
														println!("Posicao ocupada com {}, tente outra posicao", x);
														i -= 1;
													},
							_ => arr[input*2] = Tabuleiro::Jogada("X".to_string()),
						};
						
					}

					else
					{
						match &arr[input*2]
						{
							Tabuleiro::Jogada(o) =>	{
														println!("Posicao ocupada com {}, tente outra posicao", o);
														i -= 1;
													},
							_ => arr[input*2] = Tabuleiro::Jogada("O".to_string()),
						};
					}
				}

				else
				{
					println!("\n\nInsira um numero dentro dos limites");
					i -= 1;
				}

				println!();
				let mut j = 0;

				for elemento in &arr
				{
					match elemento
					{
						Tabuleiro::Coordenadas(coordenadas) => print!("{}", coordenadas),
						Tabuleiro::Linha(linha) => print!("{}", linha),
						Tabuleiro::Jogada(jogada) => print!("{}", jogada),
					};

					if j == 5 || j == 11 || j == 17
					{
						println!();
					}

					j += 1;
				}

				let mut verificacao: Vec<String> = Vec::new();

				for elemento in &arr
				{
					verificacao.push(match elemento
										{
											Tabuleiro::Jogada(jogada) => jogada.to_string(),
											_ => "0".to_string(),
										})
				}

				if i%2 == 0 && (verificacao[0*2] == "X".to_string() && verificacao[1*2] == "X".to_string() && verificacao[2*2] == "X".to_string() || verificacao[0*2] == "X".to_string() && verificacao[4*2] == "X".to_string() && verificacao[8*2] == "X".to_string() || verificacao[0*2] == "X".to_string() && verificacao[3*2] == "X".to_string() && verificacao[6*2] == "X".to_string() || verificacao[1*2] == "X".to_string() && verificacao[4*2] == "X".to_string() && verificacao[7*2] == "X".to_string() || verificacao[2*2] == "X".to_string() && verificacao[5*2] == "X".to_string() && verificacao[8*2] == "X".to_string() || verificacao[3*2] == "X".to_string() && verificacao[4*2] == "X".to_string() && verificacao[5*2] == "X".to_string() || verificacao[6*2] == "X".to_string() && verificacao[2*2] == "X".to_string() && verificacao[8*2] == "X".to_string() || verificacao[6*2] == "X".to_string() && verificacao[4*2] == "X".to_string() && verificacao[2*2] == "X".to_string())
				{
					println!("\nX Vence");
					break;
				}

				else if i%2 !=0 && (verificacao[0*2] == "O".to_string() && verificacao[1*2] == "O".to_string() && verificacao[2*2] == "O".to_string() || verificacao[0*2] == "O".to_string() && verificacao[4*2] == "O".to_string() && verificacao[8*2] == "O".to_string() || verificacao[0*2] == "O".to_string() && verificacao[3*2] == "O".to_string() && verificacao[6*2] == "O".to_string() || verificacao[1*2] == "O".to_string() && verificacao[4*2] == "O".to_string() && verificacao[7*2] == "O".to_string() || verificacao[2*2] == "O".to_string() && verificacao[5*2] == "O".to_string() && verificacao[8*2] == "O".to_string() || verificacao[3*2] == "O".to_string() && verificacao[4*2] == "O".to_string() && verificacao[5*2] == "O".to_string() || verificacao[6*2] == "O".to_string() && verificacao[2*2] == "O".to_string() && verificacao[8*2] == "O".to_string() || verificacao[6*2] == "O".to_string() && verificacao[4*2] == "O".to_string() && verificacao[2*2] == "O".to_string())
				{
					println!("\nO Vence");
					break;
				}

				i += 1;
			}
			if i== 9
			{
				println!("\nEmpate");
			}
		}
	}
}

#[derive(Debug)]
enum Tabuleiro
{
	Coordenadas(usize),
	Linha(String),
	Jogada(String),
}
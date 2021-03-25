use std::io::Read;
use std::io;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;

fn main()
{
    //panic!("crash and burn");

    //aqui vai gerar um erro porque nao consegue abrir o ficheiro e tal como o panic!(), mostra a msg escrita
	/*let f_expt = File::open("hello.txt").expect("Failed to open hello.txt");
	println!("{:?}", f_expt);*/

    let f = match File::open("hello.txt")
    {
    	Ok(file) => file,
    	Err(error) => match error.kind()
    					{
    						ErrorKind::NotFound => match File::create("hello.txt")
						    						{
						    							Ok(fc) => fc,
						    							Err(e) => panic!("Problema ao crear o ficheiro {:?}", e),
						    						}
    						other_error =>  {
    											panic!("Problema a abrir o ficheiro {:?}", other_error);
    										}
    					},
    };
    println!("{:?}", f);

    //outra forma de fazer seria usando if's

    let f_if = File::open("hello.txt").unwrap_or_else(|error|
    {
        if error.kind() == ErrorKind::NotFound
        {
            File::create("hello.txt").unwrap_or_else(|error|
            {
                panic!("Problem creating the file: {:?}", error);
            })
        }

        else
        {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("{:?}", f_if);

    //se colocar esta linha de codigo no inico vai dar erro porque nao tem o ficheiro e nao consegue criar um ficheiro
    //apos ser criado o ficheiro nas duas partes do codigo a cima ja se consegue abrir um ficheiro aqui
    let f_unwrp = File::open("hello.txt").unwrap();
    println!("{:?}", f_unwrp);

    match read_username_from_file()
    {
		Ok(func) => print!("{}", func),
					
		Err(e) => panic!("erro ao ler o ficheiro: {:?}", e),
	}

	match read_username_from_file_short()
    {
		Ok(func) => print!("{}", func),
					
		Err(e) => panic!("erro ao ler o ficheiro: {:?}", e),
	}

	match read_username_from_file_shorter()
    {
		Ok(func) => print!("{}", func),
					
		Err(e) => panic!("erro ao ler o ficheiro: {:?}", e),
	}

    match read_username_from_file_more_shorter()
    {
        Ok(func) => print!("{}", func),
                    
        Err(e) => panic!("erro ao ler o ficheiro: {:?}", e),
    }
    
}

/*#[derive(Debug)]
enum Result<T, E>
{
    Ok(T),
    Err(E),
}*/

fn read_username_from_file() -> Result<String, io::Error>
{
	let mut f = match File::open("hello.txt")
	{
		Ok(file) => file,
		Err(e) => return Err(e),
	};

	let mut s = String::new();

	match f.read_to_string(&mut s)
	{
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}
}

fn read_username_from_file_short() -> Result<String, io::Error>
{
	let mut f = File::open("hello.txt")?;
	let mut s = String::new();
	f.read_to_string(&mut s)?;
	Ok(s)
}

fn read_username_from_file_shorter() -> Result<String, io::Error>
{
	let mut s = String::new();
	File::open("hello.txt")?.read_to_string(&mut s)?;
	Ok(s)
}

fn read_username_from_file_more_shorter() ->Result<String, io::Error>
{
    fs::read_to_string("hello.txt")
}
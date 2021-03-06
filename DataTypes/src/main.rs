fn main()
{
	//Integers
	//problems with pow function
	/*let min_value_i8 : i8 = -(i8::pow(2, 8-1));
	let max_value_i8 : i8 = 2i8.pow(8-1)-1;
	let min_value_u_type : u8 = 0;
	let max_value_u8 : u8 = 2u8.pow(8)-1;
	let min_value_i16 : i16 = -(2i16.pow(16-1));
	let max_value_i16 : i16 = 2i16.pow(16-1);
	let max_value_u16 : u16 = 2u16.pow(16)-1;
	let min_value_i32 : i32 = -(2i32.pow(32-1));
	let max_value_i32 : i32 = 2i32.pow(32-1);
	let max_value_u32 : u32 = 2u32.pow(32)-1;
	let min_value_i64 : i64 = -(2i64.pow(64-1));
	let max_value_i64 : i64 = 2i64.pow(64-1);
	let max_value_u64 : u64 = 2u64.pow(64)-1;
	let min_value_i128 : i128 = -(2i128.pow(128-1));
	let max_value_i128 : i128 = 2i128.pow(128-1);
	let max_value_u128 : u128 = 2u128.pow(128)-1;*/

	println!("Integers");
	/*println!("min_value_i8: {:?}", min_value_i8);
	println!("max_value_i8: {:?}", max_value_i8);
	println!("min_value_i16: {:?}", min_value_i16);
	println!("max_value_i16: {:?}", max_value_i16);
	println!("min_value_i32 {:?}", min_value_i32);
	println!("max_value_i32: {:?}", max_value_i32);
	println!("min_value_i64: {:?}", min_value_i64);
	println!("max_value_i64: {:?}", max_value_i64);
	println!("min_value_i128: {:?}", min_value_i128);
	println!("max_value_i128: {:?}", max_value_i128);println!("max_value_i8: {:?}", max_value_i8);
	println!("min_value_i16: {:?}", min_value_i16);
	println!("max_value_i16: {:?}", max_value_i16);
	println!("min_value_i32 {:?}", min_value_i32);
	println!("max_value_i32: {:?}", max_value_i32);
	println!("min_value_i64: {:?}", min_value_i64);
	println!("max_value_i64: {:?}", max_value_i64);
	println!("min_value_i128: {:?}", min_value_i128);
	println!("max_value_i128: {:?}", max_value_i128);

	println!("min_value_u_type: {:?}", min_value_u_type);
	println!("max_value_u8: {:?}", max_value_u8);
	println!("max_value_u16: {:?}", max_value_u16);
	println!("max_value_u32: {:?}", max_value_u32);
	println!("max_value_u64: {:?}", max_value_u64);
	println!("max_value_u128: {:?}", max_value_u128);

	println!("min_value_u_type: {:?}", min_value_u_type);
	println!("max_value_u8: {:?}", max_value_u8);
	println!("max_value_u16: {:?}", max_value_u16);
	println!("max_value_u32: {:?}", max_value_u32);
	println!("max_value_u64: {:?}", max_value_u64);
	println!("max_value_u128: {:?}", max_value_u128);*/

	//tupulos
	let x: (i32, f64, u8) = (500, 6.4, 1);

    println!("\nTuplos \n1º elemento do tuplo: {}\n2ºelemento do tuplo: {}\n3º elemento do tuplo: {}", x.0, x.1, x.2);


    //arrays
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let d = [3; 5];
    let c = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    println!("\nArrays \narray without Data Type:");
    println!("array completo:  {:?}", a);
    println!("array[0]: {:?}", a[0]);
    println!("array[1]: {:?}", a[1]);
    println!("array[2]: {:?}", a[2]);
    println!("array[3]: {:?}", a[3]);
    println!("array[4]: {:?}", a[4]);

    println!("array completo: {:?}", c);
    println!("array[0]: {:?}", c[0]);
    println!("array[1]: {:?}", c[1]);
    println!("array[2]: {:?}", c[2]);
    println!("array[3]: {:?}", c[3]);
    println!("array[4]: {:?}\n   .\n   .\n   .", c[4]);

    println!("array with Data Type(integer com sinal e 32 bits) and with size(5):");
    println!("array completo: {:?}", b);
    println!("array[0]: {:?}", b[0]);
    println!("array[1]: {:?}", b[1]);
    println!("array[2]: {:?}", b[2]);
    println!("array[3]: {:?}", b[3]);
    println!("array[4]: {:?}", b[4]);

    println!("easy way to wrigth an array with the same value:\n{:?}", d);

    //function
    println!("\nFuncoes");
    let x: i32 = function(5, 5.5, 'z', "Filipe", "Alfaiate".to_string(), true);
    println!("this is a return from a function: {}", x);


    //controlos de fluxo (if's e loops)
    println!("\nIf expressions");
    
    let third: u32 =3;
    if third > 5
    {
    	println!("return of if: {}", false);
    }

    else if third == 0 {
    	println!("return of if: {:?}", false);
    }

    else if third != 3 || (third > 5 && third == 0)
    {
    	println!("return of if: {:?}", false);
    }
    else {
    	println!("return of if: {:?}", true);
    }

    let condition: bool = false;
    let number = if condition {1} else {0}; //the return value has to be the same type, ex.: &str != "".to_string()
    println!("difrent kind of if: {}", number);


    //loops
    println!("\nLoops");

    let mut i: u32 = 0;

    loop 
    {
    	if i > 5
    	{
    		break;
    	}

    	println!("loop increment: {:?}", i);

    	i += 1;
 
    };

    let mut counter = 0;
    let result = 
    loop
    {
        if counter == 5
        {
            break counter;
        }
        counter += 1;
    };

    println!("other types of loop {}", result);


    //while
    println!("\nLoop with conditions (while)");

    while counter != 0
    {
    	println!("Not yet, counter still: {}", counter);
    	counter -= 1;
    }
    println!("termineted the while with counter: {}", counter);


    println!("\nPercorrer arrays com while");
    let a = [1, 10, 100, 1000];
    let mut index = 0;

    while index < a.len()
    {
    	println!("posicao a[{:?}] = {}", index, a[index]);
    	index += 1;
    }


    println!("\nPercorrer array com for");

    for element in a.iter()
    {
    	println!("elemento do array: {:?}", element);
    }
    println!("Same result but with a much cleaner code");


    println!("\nCountdown with  for");

    for num in (1..10).rev()
    {
    	println!("array inverso: {:?}", num);
    }

    println!("\n**STUDY TERMINETED FOR TODAY**");


}

//&str = pointer que indica o incio de uma string
//-> i32 é o tipo do return
fn function(x: i32, y: f32, z: char, w: &str, k: String, a: bool) -> i32
{
	println!("this print is inside 'function' with argument: {}, {}, {}, {} {}, {}", x, y, z, w, k, a);
	let y: i32 = 4;
	return y;
	//or just y
}

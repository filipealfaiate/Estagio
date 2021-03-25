mod front_of_house
{
    pub mod hosting 
    {
    	pub fn add_to_waitlist()
        	{println!("added to waitting list");}

        fn seat_at_table()
        	{println!("seat at the table");}
    }

    pub mod serving
    {
        fn take_order()
        	{println!("take order");}

        fn serve_order()
        	{println!("serve order");}

       	pub mod back_of_house
       	{
       		pub fn fix_incorrect_order()
       		{
       			cook_order();
       			super::serve_order;
       		}

       		fn cook_order()
       			{println!("it's fixed");}

       		#[derive(Debug)]
       		pub struct Breakfast
		    {
		        pub toast: String,
		        seasonal_fruit: String,
		    }

		    impl Breakfast
		    {
		        pub fn summer(toast: &str) -> Breakfast
		        {
		            Breakfast
		            {
		                toast: String::from(toast),
		                seasonal_fruit: String::from("peaches"),
		            }
		        }
		    }

		    #[derive(Debug)]
		    pub enum Appetizer
		    {
		        Soup,
		        Salad,
		    }
       	}

        fn take_payment()
        	{println!("paytime");}
    }
}

pub fn eat_at_restaurant()
{
	//caminho absuloto, nao funciona nao sei porque
	//crate::front_of_house::hosting::add_to_waitlist();

	//caminho relativo
	front_of_house::hosting::add_to_waitlist();
	front_of_house::serving::back_of_house::fix_incorrect_order();

	//STRUCTS

    let mut meal = front_of_house::serving::back_of_house::Breakfast::summer("Rye");
    //troca Rye por Wheat
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    println!("a struct tem os seguintes elementos\n{:#?}", meal);

    // The next line won't compile if we uncomment it; we're not allowed
    // because seasonal_fruit is private
    // meal.seasonal_fruit = String::from("blueberries");

    //ENUMS

    let order1 = front_of_house::serving::back_of_house::Appetizer::Soup;
	let order2 = front_of_house::serving::back_of_house::Appetizer::Salad;

	println!("{:?}, {:?}", order1, order2);

}
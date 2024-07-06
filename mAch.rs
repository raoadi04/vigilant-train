fn main() {
	let number = 13;
	//TODO ^ Try different values for 'number'
	
	println!("Tell me about {}", number);
	match number {
		//Match a single value
		1 => println!("One!"),
		
		//Match a single value
		2 | 3 | 5 | 7 | 11 => println!("This is a prime"),

		//Match a single value
		13..=19 => println!("A teen"),

		//Handle the rest of cases
		_=> println!("Ain't special"),
		//TODO ^ Try commenting out this catch-all arm	
	}
	
	let boolean = true;
	//Match is an expression too 
	let binary = match boolean {
		//The arms of match must cover all the possible values
		false => 0,
		true => 1,
		//TODO ^ Try commenting out one of these arms
	};
	
	println!("{} -> {}", boolean, binary);
}

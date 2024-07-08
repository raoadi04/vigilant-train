fn some_number() -> Option<u32> {
	Some(42)
}

fn main() {
	match some_number() {
		//Got 'Some' varianr, match if its value, bound to 'n',
		//is equal other number.
		Some(n @ 42) => println!("The Answer: {}!",n),
		//Match any other number.
		Some(n) => println!("Not interesting... {}",n),
		//Match anything else ('None' variant).
		_ => (),
	}
	//println!("Hello, world!");
}

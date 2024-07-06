fn main() {
	let names = vec!["Bob", "Frank", "Ferris"];
	
	for name in names.iter(){
		//interesting read the comment and try the match function(play around with it :) )
		match name {
			&"Bob" => println!("There is a rustcean among us!"),
			//TODO ^ Try deleting the & and matching just "Ferris"
			_=> println!("Hello {}", name),
		}
	}
	println!("names: {:?}", names);
}

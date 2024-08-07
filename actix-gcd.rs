use actix_web::{web, App, HttpResponse, HttpServer};

fn gcd(mut n: u64, mut m: u64) -> u64 {
	assert!(n != 0 && m != 0);
	while m != 0 {
		if m < n {
			let t = m;
			m = n;
			n = t;
		}
		m = m % n;
	}
	n
}

fn main() {
	let server = HttpServer::new(|| {
		App::new()
			.route("/", web::get().to(get_index))
	});
	println!("Serving on http://localhost:3000...");
	server
		.bind("127.0.0.1:3000").expect("error binding server to address")
		.run().expect("error running server");
}

fn get_index() -> HttpResponse {
	HttpResponse::Ok()
		.content_type("text/html")
		.body(
			r#"
				<title>GCD Calculator</title>
				<form action = "/gcd" method = "post">
				<input type = "text" name = "n"/>
				<input type = "text" name = "m"/>
				<button type = "submit">Compute GCD</button>
				</form>
			"#,
		)
}

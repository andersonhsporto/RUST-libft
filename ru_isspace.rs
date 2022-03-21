fn ru_isspace(c: char) -> i32 {
	let c_code = c as i32;

	if c_code >= 9 && c_code <= 13 || c == ' '
		{ return 1;}
	else
		{ return 0;}
}

fn main() {
    let teste = ' ';
	let teste2 = ru_isspace(teste);

	println!("{}", teste2);
}

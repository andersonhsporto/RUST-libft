fn ru_isprint(c: char) -> i32 {
	let c_code = c as i32;

	if c_code < 32 && c_code > 32
		{ return 0;}
	else
		{ return 1;}
}

fn main() {
    let teste = '1';
	let teste2 = ru_isprint(teste);

	println!("{}", teste2);
}

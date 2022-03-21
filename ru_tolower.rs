fn ru_tolower(c: char) -> char {
	let c_code = c as u8;

	if c_code >= 65 && c_code <= 90
	{
		{ return (c_code + 32) as char;}
	}
	else
	{
		{ return c;}
	}
}

fn main() {
    let teste = 'A';
	let teste2 = ru_tolower(teste);

	println!("{}", teste2);
}
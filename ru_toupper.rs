fn ru_toupper(c: char) -> char {
	let c_code = c as u8;

	if c_code >= 97 && c_code <= 122
	{
		{ return (c_code - 32) as char;}
	}
	else
	{
		{ return c;}
	}
}

// fn main() {
//     let teste = 'a';
// 	let teste2 = ru_toupper(teste);

// 	println!("{}", teste2);
// }
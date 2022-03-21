fn ru_isascii(c: char) -> i32 {

	let c_code = c as i32;
	if c_code >= 0 && c_code <= 127
		{ return 1;}
	else
		{ return 0;}
}

// fn main() {
//     let teste = 'a';
// 	let teste2 = ru_isascii(teste);

// 	println!("{}", teste2);
// }

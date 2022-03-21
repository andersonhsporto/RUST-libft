fn ru_isalnum(c: char) -> i32 {
	if ru_isalpha(c) == 1 || ru_isdigit(c) == 1
		{ return 1;}
	else
		{ return 0;}
}

// fn main() {
//     let teste = '#';
// 	let teste2 = ru_isalnum(teste);

// 	println!("{}", teste2);
// }

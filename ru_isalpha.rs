fn ru_isalpha(c: char) -> i32 {
	if !(('z' >= c && c >= 'a') || ('Z' >= c && c >= 'A'))
		{ return 0;}
	else
		{ return 1;}
}

// fn main() {
//     let teste = 'a';
// 	let teste2 = ru_isalpha(teste);

// 	println!("{}", teste2);
// }

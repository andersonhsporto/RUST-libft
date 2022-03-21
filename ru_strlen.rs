fn ru_strlen(str: String) -> usize {
	let mut index = 0;

	for _c in str.chars() {
		index += 1;
	}
	{ return index;}
}

// fn main() {
// 	let teste2 = ru_strlen("teste".to_string());
// 	println!("{}", teste2);
// }

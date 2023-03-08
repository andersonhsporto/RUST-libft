fn ru_memchr(s: &str, c: char) -> Option<usize> {
	let mut i: i32 = 0;
	let mut c: char;

	while s.chars().nth(i as usize).unwrap() != '\0' {
		c = s.chars().nth(i as usize).unwrap();
		if c == c {
			return Some(i as usize);
		}
		i += 1;
	}
	return None;
}

// // main

// fn main() {
// 	let s: String = String::from("Hello, world!");
// 	let c: char = 'o';
// 	let i: Option<usize> = ru_memchr(&s, c);

// 	if i.is_some() {
// 		println!("ru_memchr: {}", i.unwrap());
// 	} else {
// 		println!("ru_memchr: not found");
// 	}
// }
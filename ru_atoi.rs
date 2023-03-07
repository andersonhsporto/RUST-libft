fn ru_atoi(s: &str) -> i32 {
	let mut i: i32 = 0;
	let mut sign: i32 = 1;
	let mut n: i32 = 0;
	let mut c: char;

	while s.chars().nth(i as usize).unwrap() == ' ' {
		i += 1;
	}
	if s.chars().nth(i as usize).unwrap() == '-' {
		sign = -1;
		i += 1;
	}
	while s.chars().nth(i as usize).unwrap() != '\0' {
		c = s.chars().nth(i as usize).unwrap();
		if ru_isdigit(c) == 0 {
			break;
		}
		n = n * 10 + (c as i32 - '0' as i32);
		i += 1;
	}
	return sign * n;
}

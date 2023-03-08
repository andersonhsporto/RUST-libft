fn ru_itoa(n: i32) -> String {
	let mut i: i32 = 0;
	let mut j: i32 = 0;
	let mut k: i32 = 0;
	let mut c: char;
	let mut s: String = String::new();
	let mut v: Vec<char> = Vec::new();

	if n < 0 {
		s.push('-');
		i = -n;
	} else {
		i = n;
	}
	while i > 0 {
		v.push((i % 10 + '0' as i32) as u8 as char);
		i /= 10;
	}
	while j < v.len() {
		k = v.len() - j - 1;
		c = v[j as usize];
		v[j as usize] = v[k as usize];
		v[k as usize] = c;
		j += 1;
	}
	for c in v {
		s.push(c);
	}
	return s;
}
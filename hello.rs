// hello.rs

// fn slice() {
// 	let ints = [1,2,3,4,5];
// 	let slice = &ints[0..4];
// 	let first = slice.get(0);
// 	let last = slice.get(5);

// 	println!("slice {:?}", slice);
// 	println!("first {}", first.unwrap());
// 	println!("last {}", last.is_some());
// }

// fn arr() {
// 	let mut arr = [1,2,3,4];
// 	println!("arr {:?}", arr_modifies(&mut arr));
// }

// fn arr_modifies(arr: &mut [i32]) -> &mut [i32] {
// 	arr[2] = 0;
// 	arr
// }

// fn vector() {
// 	let mut v = Vec::new();
// 	v.push(10);
// 	v.push(20);
// 	v.push(30);

// 	let first = v[0];
// 	let maybe_first = v.get(0);

// 	println!("v is {:?}", v);
// 	println!("first is {}", first);
// 	println!("maybe_first is {:?}", maybe_first);
// }

// fn pro_sum() {
// 	let sum: i32 = (0..5).sum();
// 	println!("sum was {}", sum);
	
// 	let sum: i64 = [10, 20, 30].iter().sum();
// 	println!("sum was {}", sum);

// }

// fn slice_windows_and_chunks() {
// 	let ints = [1, 2, 3, 4, 5];
// 	let slice = &ints;

// 	for s in slice.windows(2) {
// 		println!("window {:?}", s);
// 	}
//  	for s in slice.chunks(2) {
// 		println!("chunks {:?}", s);
// 	}
// }

// fn more_vectors() {
// 	let mut v1 = vec![10, 20, 30, 40];
// 	v1.pop();

// 	let mut v2 = Vec::new();
// 	v2.push(10);
// 	v2.push(20);
// 	v2.push(30);

// 	assert_eq!(v1, v2);

// 	v2.extend(0..2);
// 	assert_eq!(v2, &[10, 20, 30, 0, 1]);
// }

// fn mooore_vectors() {
// 	let mut v1 = vec![1, 10, 5, 1, 2, 11, 2, 40];
// 	v1.sort();
// 	v1.dedup();
// 	assert_eq!(v1, &[1, 2, 5, 10, 11, 40]);
// 	let v2 = v1.clone();

// 	v1.insert(3,70);
// 	assert_eq!(v1, &[1, 2, 5, 70, 10, 11, 40]);

// 	v1.remove(3);
// 	assert_eq!(v1, &[1, 2, 5, 10, 11, 40]);
// 	assert_eq!(v1, v2);

// 	v1.clear();
// 	assert_eq!(v1, &[]);

// }

// fn dump_for_strings_fn(s: &str) {
// 	println!("str '{}'", s);
// }

// fn strings() {
// 	let text = "hello dolly";
// 	let s = text.to_string();

// 	dump_for_strings_fn(text);
// 	dump_for_strings_fn(&s);
// }

// fn string_push_pop() {
// 	let mut s = String::new();

// 	s.push('H');
// 	s.push_str("ello");
// 	s.push(' ');
// 	s += "World!";
// 	s.pop();

// 	assert_eq!(s, "Hello World");
// }

// fn array_to_str() {
// 	let arr = [10, 20, 30];
// 	let mut res = '['.to_string();
// 	for v in arr {
// 		res += &v.to_string();
// 		res.push(',');
// 	}
// 	res.pop();
// 	res.push(']'); 
// 	// res should be "[10,20,30]" now

// 	let res = format!("hello {}", res);
// 	assert_eq!(res, "hello [10,20,30]");
// }

// fn string_slices() {
// 	let text = "static";
// 	let string = "dynamic".to_string();

// 	let text_s = &text[1..];
// 	let string_s = &string[2..4];

// 	println!("slices {:?} {:?}", text_s, string_s);
// }

// fn string_length() {
// 	let multilingual = "Hi! ¡Hola! привет!";
// 	for ch in multilingual.chars() {
// 		print!("'{}' ", ch);
// 	}
// 	println!("");
// 	println!("len {}", multilingual.len());
// 	println!("count {}", multilingual.chars().count());

// 	let maybe = multilingual.find('п');
// 	if maybe.is_some() {
// 		let hi = &multilingual[maybe.unwrap()..];
// 		println!("Russian hi {}", hi);
// 	}
// }

// fn string_split() {
// 	let text = "the red fox and the lazy dog";
// 	let words: Vec<&str> = text.split_whitespace().collect();
// 	println!("words {:?}", words);
	
// 	let mut words = Vec::new();
// 	words.extend(text.split_whitespace());
// 	println!("words {:?}", words);

// 	let stripped: String = text.chars()
// 		.filter(|ch| ! ch.is_whitespace()).collect();
// 	println!("stripped {:?}", stripped);
// }

fn main() {
	// slice();
	// arr();
	// vector();
	// pro_sum();
	// slice_windows_and_chunks();
	// more_vectors();
	// mooore_vectors();
	// strings();
	// string_push_pop();
	// array_to_str();
	// string_slices();
	// string_length();
	// string_split();

}
macro_rules! a_macro {
	() => (
		println!("Hello from macro");
	)
}

// expr is a designator type, $e can be any expression
macro_rules! x_and_y {
// These act like a match statement
	(x => $e:expr) => (println!("X: {}", $e));
	(y => $e:expr) => (println!("Y: {}", $e));
}

// ident is an identifier which is typically a function name or var name
macro_rules! build_fn {
	($func_name:ident) => (
		fn $func_name() {
			println!("Called {:#?}()", stringify!($func_name));
		}
	)
}

// Again printing the stringified version of the expression as well as computing it
macro_rules! print_ex {
	($e:expr) => (
		println!("{:?} = {:?}", stringify!($e), $e);
	)
}

macro_rules! overload {
	($l:expr; and $r:expr) => (
		println!("{:?} and {:?} is {:?}", stringify!($l), stringify!($r), $l && $r);
	);

	($l:expr; or $r:expr) => (
		println!("{:?} or {:?} is {:?}", stringify!($l), stringify!($r), $l || $r);
	);
}

// List comp macro
macro_rules! compr {
	($id1: ident | $id2: ident <- [$start: expr ; $end: expr] , $cond: expr) => {
		{
			let mut vec = Vec::new();

			for num in $start..$end + 1 {
				if $cond(num) {
					vec.push(num);
				}
			}

			vec
		}
	}
}

macro_rules! new_map {
	// ($($key: expr => $val: expr),*) => {
	// Change to + because expression has to happen at least once
	($($key: expr => $val: expr),+) => {
		{
			use std::collections::HashMap;
			let mut map = HashMap::new();

			// repeat as many times as there are more expressions
			$(
				map.insert($key, $val);
			)*

			map
		}
	}
}

fn main() {
	a_macro!();
	x_and_y!(x => 10);
	x_and_y!(y => 10 + 20);
	build_fn!(hello);
	hello();

	print_ex!({
		let y = 20;
		let x = 10;
		x + y + 10 * 3 * 100
	});

	overload!(true; and false);
	overload!(false; or true);

	fn even(x: i32) -> bool {
		x % 2 == 0
	}

	let evens = compr![x | x <- [1;10], even];
	println!("{:?}", evens);

	let map = new_map!{5 => 7, 9 => 2, 11 => 22};
	println!("{:?}", map);
}

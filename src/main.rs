macro_rules! a_macro {
	() => (
		println!("Hello from macro");
	)
}

// expr is a designator type, $e can be any expression
macro_rules! x_and_y {
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

fn main() {
	a_macro!();
	x_and_y!(x => 10);
	x_and_y!(y => 10 + 20);
	build_fn!(hello);
	hello();
}

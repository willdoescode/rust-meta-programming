macro_rules! a_macro {
	() => (
		println!("Hello from macro");
	)
}

macro_rules! x_and_y {
	(x => $e:expr) => (println!("X: {}", $e));
	(y => $e:expr) => (println!("Y: {}", $e));
}

fn main() {
	a_macro!();
	x_and_y!(x => 10);
	x_and_y!(y => 10 + 20);
}

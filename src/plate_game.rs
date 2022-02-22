use std::cmp;
pub fn play(plate_count:usize) {
	let mut s1 = Stack::new(plate_count, String::from("Start"));
	let mut s2 = Stack::new(0, String::from("Destination"));
	let mut s3 = Stack::new(0, String::from("Buffer"));

	let stack_size = plate_count;

	print_stacks(&s1, &s2, &s3);

	solve(stack_size, &mut s1, &mut s2, &mut s3);

	print_stacks(&s1, &s2, &s3);
}

fn solve(stack_size: usize, start: &mut Stack, dest: &mut Stack, buf: &mut Stack) {
	if stack_size == 1 {
		start.move_top(dest);
		println!("Moved plate {} from {} to {}", dest.peek(), start.name, dest.name);
	} else {
		solve(stack_size-1, start, buf, dest);
		start.move_top(dest);
		println!("Moved plate {} from {} to {}", dest.peek(), start.name, dest.name);
		solve(stack_size-1, buf, dest, start);
	}
}

fn print_stacks(s1: &Stack, s2: &Stack, s3: &Stack) {
	let max_stack_size = cmp::max(cmp::max(s1.size(), s2.size()), s3.size());
	let max_plate_size = s1.size() + s2.size() + s3.size();
	for n in (0..max_stack_size).rev() {
		let (p1, p2, p3);
		if s1.size() <= n {
			p1 = 0;
		} else {
			p1 = s1.stack[n];
		}
		if s2.size() <= n {
			p2 = 0;
		} else {
			p2 = s2.stack[n];
		}
		if s3.size() <= n {
			p3 = 0;
		} else {
			p3 = s3.stack[n];
		}
		println!("{}{} | {}{} | {}{}", 
			"#".repeat(p1), " ".repeat(max_plate_size - p1),
			"#".repeat(p2), " ".repeat(max_plate_size - p2),
			"#".repeat(p3), " ".repeat(max_plate_size - p3));
	}
	println!("{}", "-".repeat(max_plate_size * 3 + 6))
}

struct Stack {
	stack: Vec<usize>,
	name: String,
}
impl Stack {
	pub fn new(size: usize, name: String) -> Self {
		let mut new_stack = Vec::new();
		for n in (1..=size).rev() {
			new_stack.push(n)
		}
		Stack { 
			stack: new_stack,
			name }
	}

	pub fn move_top(&mut self, dest: &mut Stack) {
		if dest.stack.is_empty() || self.peek() < dest.peek() {
			dest.stack.push(self.stack.pop().unwrap());
		} else {
			panic!("Made an invalid move");
		}
	}

	pub fn size(&self) -> usize {
		self.stack.len()
	}

	fn peek(&self) -> &usize {
		if self.stack.is_empty() {
			panic!("Peeked an empty stack!")
		}
		&self.stack[self.stack.len()-1]
	}
}
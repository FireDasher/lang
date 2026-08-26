# My New Unnamed Programming Language (unfinished)

It's my new, unnamed programming language. It's like C++ but better.

I need names for it! Z is the placeholder

## Planned Features:
- Type is after like Rust, ex. `let value: i32 = 42`, or inferred if no type is after the name, ex. `let value = 42`
- Functions are like `fn functionName(paramater_one: ParamaterOneType, paramater_two: ParamaterTwoType) return_type {...}`
- You can ommit the return value of a function to return nothing
- Types are fixed-width and have names based on their size like in Rust, ex. `i32` = int, `u32` = unsigned int, `i64` = long long, `u64` = unsigned long long, `f32` = float, `f64` = double. 
- Dot for everything, including accessing members, calling methods, using stuff in a namespace, and accessing members and methods on a reference
- You can overload most operators by putting a function named operator_name of the operator you want to overload, ex. `fn operator_mul(self, other: f32) Self { Self(self.x * other, self.y * other) }`
- Rust-like struct initialization with brackets
- Python-like string formatting
- Import statements, maybe, or maybe libraries will just be defined in the package manager file and made global scope like in Rust, I don't know yet
- Single inheritance
- Compile time duck typing
- Defaults for arguments are allowed, but no overloading
- You can use the bracket initialization of structs without its name if it's inferrable, ex `fn button(label: &str, options: ButtonOptions = {}) bool {...}` then `button("Hello World", {color: 0xFF0000})`
- `&str` is like `&char` for a UTF-8 encoded string and with some string-related functions
- `String` is like `Vec<char>` for a UTF-8 encoded string and with some string-related functions

**Basic example:**
```z
fn main() {
	let number: i32 = 21;
	std.print(f"The Number is: {number * 2}"); // prints "The Number is: 42"
}
```

**Structs example:**
```z
struct Point {
	x: f32 = 0.0;
	y: f32 = 0.0;

	// Member functions
	fn member() bool {
		std.print("Called the useless member function on Point");
		true
	}
	
	// You can have a function which is called by calling the class by having a function named "construct"
	fn construct(x: f32, y: f32) Self {
		Self {x, y}
	}

	// Operator overloading is supported like this
	// The backticks are there because otherwise overloading greater than might confuse the compiler
	// Also capital Self acts as an alias for Point here, but lowercase self takes in self directly
	fn operator_add(self, other: Self) Self {
		Self(self.x + other.x, self.y + other.y)
	}

	// Use &self if you want to take self as a reference instead
	fn operator_add_assign(&self, other: Self) Self {
		self.x += other.x;
		self.y += other.y;
	}

	// used in printing
	fn format(self) String {
		f"({self.x}, {self.y})"
	}
}

fn main() {
	// When initializing with bracket notation, Uninitialized values are set to their default, unless they don't have one then it throws an error if unset
	let test = Point { y: 0.1 + 0.2 };

	let point: Point; // initalizes with defaults (0, 0)
	point += Point(1.0, 2.0);
	std.print(f"Point: {point}");
	let point2 = point + Point(-0.567465, 45.473678);
	std.print(f"Point2: {point2}");
}
```

**Classes & Inheritance example: (will probably change a lot before added)**

```z
// Classes are just like structs but with inheritance
class Node {
	name: String,
	parent: &Node = null,
	children: Vec<&Node> = Vec.new(),
	position: vec2 = vec2(0.0, 0.0),
	
	virtual fn process(&self, dt float) {}
}

class Sprite : Node {
	texture_path: String,
	
	override fn process(&self, dt float) {
		self.position.x += 10.0 * dt;
		std.print(f"Position of {self.name}: {self.position}");
	}
}

fn main() {
	let player = Sprite{name: "Player", texture_path: "assets/player.png"};
	let enemy = Sprite{name: "Enemy", position: vec2(0.0, 100.0), texture_path: "assets/enemy.png"};

	let world: Vec<&Node> = [&player, &enemy];
	
	while true {
		for node in world {
			node.process();
		}
		std.sleep(16); // in miliseconds
	}
}
```
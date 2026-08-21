# My New Unnamed Programming Language (unfinished)

It's my new, unnamed programming language. It's like C++ but better.

I need names for it! Z is the placeholder

## Planned Features:
- Type is after like Rust, ex. `let value: i32 = 42`, or inferred if no type is after the name, ex. `let value = 42`
- Functions are like `fn functionName(paramater_one: ParamaterOneType, paramater_two: ParamaterTwoType) return_type {...}`
- You can ommit the return value of a function to return nothing
- Types are fixed-width and have names based on their size like in Rust, ex. `i32` = int, `u32` = unsigned int, `i64` = long long, `u64` = unsigned long long, `f32` = float, `f64` = double. 
- Dot for everything, including accessing members, calling methods, using stuff in a namespace, and accessing members and methods on a reference
- You can overload most operators by putting a function named operator with the operator in quotes and the other type is based on the paramaters, ex. `fn operator"*"(self, other: f32) Self { Self(self.x * other, self.y * other) }`
- Rust-like struct initialization with brackets
- Python-like string formatting
- Import statements
- Single inheritance
- Compile time duck typing

**Basic example:**
```z
import std;

fn main() {
	let number: i32 = 21;
	std.print(f"The Number is: {number * 2}"); // prints "The Number is: 42"
}
```

**Structs example:**
```z
import std;
use std.String;

struct Point {
	x: f32 = 0.0;
	y: f32 = 0.0;

	// Member functions
	fn member() bool {
		std.print("Called the useless member function on Point");
		true
	}
	
	// You can overload () statically to make it act like a C++ constructor
	// Inline forces inlinement, if not explicitly included then in build mode LLVM might automatically inline small functions even if not marked inline
	inline fn operator"()"(x: f32, y: f32) Self {
		Self {x, y}
	}

	// Operator overloading is supported like this
	// The backticks are there because otherwise overloading greater than might confuse the compiler
	// Also capital Self acts as an alias for Point here, but lowercase self takes in self directly
	fn operator"+"(self, other: Self) Self {
		Self(self.x + other.x, self.y + other.y)
	}

	// Use &self if you want to take self as a reference instead
	fn operator"+="(&self, other: Self) Self {
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
import std;
import math; // Math is for vector math, stuff like square rooting and such is a core feature
use std.(Vec, String), math.vec2; // vec2 is lowercase to line up with shaders and stuff and because it's a very simple struct

class Node {
	let name: String;
	let parent: &Node = null;
	let children: Vec<&Node> = Vec.new();
	let position: vec2 = vec2(0.0, 0.0);
	
	virtual fn process(&self, dt float) {}
}

class Sprite : Node {
	let texture_path: String;

	// Classes have constructors
	fn construct(&self, name: String, position: vec2, texture_path: String) {
		self.name = name;
		self.position = position;
		self.texture_path = texture_path;
	}
	
	override fn process(&self, dt float) {
		self.position.x += 10.0 * dt;
		std.print(f"Position of {self.name}: {self.position}");
	}
}

fn main() {
	let player = Sprite("Player", vec2(0.0, 0.0), "assets/player.png");
	let enemy = Sprite("Enemy", vec2(0.0, 100.0), "assets/enemy.png");

	let world: Vec<&Node> = [&player, &enemy];
	
	while true {
		for node in world {
			node.process();
		}
		std.sleep(16); // in miliseconds
	}
}
```
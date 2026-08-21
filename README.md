# My New Unnamed Programming Language (unfinished)

It's my new, unnamed programming language. It's like C++ but better.

I need names for it! Z is the placeholder

## Planned Features:
- Type is after like go, ex. `let value int = 32`, or inferred if no type is after the name, ex. `let value = 32`
- Functions are like `fn functionName(paramater_one int, paramater_two int) int {...}`
- You can ommit the return value of a function to return nothing
- Types are fixed-width, and if you don't want numbers in your type's names, `int` is an alias for i32, `uint` is an alias for u32, `long` is an alias for i64, `ulong` is an alias for u64, `float` is an alias for f32, and `double` is an alias for f64
- Dot for everything, including accessing members, calling methods, using stuff in a namespace, and accessing members and methods on a reference
- Rust-like struct initialization with brackets
- Python-like string formatting
- Import statements
- Single inheritance
- Compile time duck typing

**Basic example:**
```z
import std;

fn main() {
	let number = 21;
	std.print(f"The Number is: {number * 2}"); // prints "The Number is: 42"
}
```

**Structs example:**
```z
import std;
use std.String;

struct Point {
	let x float = 0.0;
	let y float = 0.0;

	// Member functions
	fn member() bool {
		std.print("Called the useless member function on Point");
		true
	}
	
	// You can overload () to make it act like a C++ constructor
	// Inline forces inlinement, if not explicitly included then in build mode LLVM might automatically inline small functions even if not marked inline
	inline fn operator<`()`>(x float, y float) Self {
		Self {x, y}
	}

	// Operator overloading is supported like this
	// The backticks are there because otherwise overloading greater than might confuse the compiler
	// Also capital Self acts as an alias for Point here, but lowercase self takes in self directly
	fn operator<`+`, Self>(self, other Self) Self {
		Self(self.x + other.x, self.y + other.y)
	}

	// Use &self if you want to take self as a reference instead
	fn operator<`+=`, Self>(&self, other Self) Self {
		self.x += other.x;
		self.y += other.y;
	}

	// used in printing
	fn format(self) String {
		f"({self.x}, {self.y})"
	}
}

fn main() {
	let point Point; // initalizes with defaults (0, 0)
	point += Point(1.0, 2.0);
	std.print(f"Point: {point}");
	let point2 = point + Point(-0.567465, 45.473678);
	std.print(f"Point2: {point2}");
}
```

**Inheritance example:**

```z
import std;
import math; // Math is for vector math, stuff like square rooting and such is a core feature
use std.(Vec, String), math.vec2; // vec2 is lowercase to line up with shaders and stuff and because it's a very simple struct

class Node {
	let name String;
	let parent &Node = null;
	let children Vec<&Node> = Vec.new(); // functions in defaults are called when it's created unless you wrap it in a const block or something, but then that would probably cause a segmentation fault for vecs because the pointer is only valid at compile time
	let position vec2 = vec2(0.0, 0.0);
	virtual fn process(&self, dt float) {}
}

class Sprite : Node {
	let texture_path String;
	override fn process(&self, dt float) {
		self.position.x += 10.0 * dt;
		std.print(f"Position of {self.name}: {self.position}");
	}
}

fn main() {
	let player = Sprite{ name: "Player", texture_path: "assets/player.png" }; // Uninitialized values are set to their default, unless they don't have one then it throws an error if unset
	let enemy = Sprite{ name: "Enemy", position: vec2(0.0, 100.0), texture_path: "assets/enemy.png" };

	let world Vec<&Node> = [&player, &enemy];
	
	while true {
		for node in world {
			node.process();
		}
		sleep(100); // in miliseconds, when I actually make this sleep wouldn't be in global scope
	}
}
```
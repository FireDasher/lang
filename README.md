# My New Unnamed Programming Language (unfinished)

It's my new, unnamed programming language. It's like C++ but better.

I need names for it! Z is the placeholder

## Planned Features:
- C-like syntax
- Pointers types are &type and get address by &variable_you_want_the_address_of
- Have as many constructors as you want with different names
- Create a constructor with the name default to be able to do Class(args) to construct it
- Create structs with a Rust-like initializer syntax
- Infer size and sign of types (32-bit or 64-bit, signed or unsigned) and whether a string literal is &char or String
- Single inheritance, classes can inherit and be inherited but structs can't
- Only inheritance can be polymorphized
- Duck typing at compile time
- Clean module system that doesn't require separating into a header and source file
- No memory safety
- Natively compatible with C and C++ libaries
- Powerful macros

### Example:
```z
import std;

void main() {
	std::print("Hello, World!");
}
```

### Inheritance example:
```z
import std;
import vectormath;
use vectormath::vec2;

class Node {
	String name;
	virtual void update(float delta) {}
}

class Node2D : Node {
	vec2 position;
	vec2 scale;
	float rotation;
}

void main() {
	Vec<&Node> nodes = [
		&Node2D {
			name: "Player",
			position: vec2(0.0, 0.0),
			scale: vec2(1.0, 1.0),
			rotation: 0.0,
		},
		&Node {
			name: "A node"
		}
	];
	for node in nodes {
		printf("Found node: {node.name}");
	}
}
```

### Duck typing example
```z
V add<T, U, V>(T first, U second) {
	(first.floor() + second.ceiling() as T) as V
}

void main() {
	f32 first = 1.1; // infer
	int result = add(first, 2.3_f64); // or use f64 suffix, underscore for readability
	printf("Result: {result}"); // 4
}
```

### Structs and constructor example
```z
struct Point {
	f32 x;
	f32 y;
	constructor default(f32 x, f32 y) {
		self.x = x;
		self.y = y;
	}
	constructor scalar(f32 scalar) {
		self.x = scalar;
		self.y = scalar;
	}
}

void main() {
	Point pointA = Point(3.0, 5.0);
	Point pointB = Point(-3.14159);
	Point pointC = Point { x: 1.1, y: 2.2 };
}
```
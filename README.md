# My New Unnamed Programming Language (unfinished)

It's my new, unnamed programming language. It's like C++ but better.

I need names for it! Z is the placeholder

### Example:
```z
import std;

void main() {
	std::print("Hello, World!");
}
```

## Features:
- C-like syntax
- Pointers types are &type and get address by &
- Have as many constructors as you want with different names
- Create a constructor with the name default to be able to do Class(args) to construct it
- Create structs with a Rust-like syntax
- Infer size and sign of types (32-bit or 64-bit, signed or unsigned) and whether a string literal is &char or String
- Single inheritance, classes can inherit and be inherited but structs can't
- Traits, both classes and structs can implement an infinite number of them, the only difference is they can only have functions not members
- Clean module system that doesn't require separating into a header and source file
- No memory safety
- Natively compatible with C and C++ libaries

### More example:
```z
import std;

class Entity {
	u32 age;
	virtual void speak();
}

trait Living {
	void breathe();
}

class Person : Entity, Living {
	String name;
	override void speak() {
		std::printf("Hello, it's me {name}");
	}

	constructor default(String name, u32 age) {
		self.name = name;
		self.age = age;
	}

	impl void breathe() {
		std::print("Breathing sounds");
	}
}

struct Point : Add<Self>, Mul<f32> {
	f32 x;
	f32 y;
	impl Self add(Self a, Self b) {
		Self( a.x + b.x, a.y + b.y )
	}
	impl Self mul(f32 scalar) {
		Self(a.x * scalar, a.y * scalar)
	}
}

void main() {
	&Entity entity = &Person("Sfdkjk", 2);
	entity.speak();
	Point point = (Point(10.0, 72.5) + Point(PI, -54.2)) * 0.5;
	std::debug(point);
}
```
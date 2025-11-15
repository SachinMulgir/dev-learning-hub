# 🦀 Structs

## 1. **What is a Struct?**
A **struct** in Rust is a custom data type that groups related data together.  
Similar to classes, but **without methods inside the struct definition**.

---

## 2. **Named-Field Struct**

```rust
struct User {
    username: String,
    age: u32,
    active: bool,
}
```

### Creating an instance:
```rust
let u = User {
    username: String::from("Sachin"),
    age: 25,
    active: true,
};
```

### Accessing fields:
```rust
println!("{}", u.username);
```

---

## 3. **Mutable Struct Instance**

```rust
let mut u = User {
    username: String::from("Sachin"),
    age: 25,
    active: true,
};

u.age = 26;
```

---

## 4. **Struct Update Syntax**

```rust
let u2 = User {
    username: String::from("Rahul"),
    ..u
};
```

⚠️ Moves data from `u` (if fields are non-Copy), making `u` unusable.

---

## 5. **Tuple Structs**

```rust
struct Color(i32, i32, i32);
```

### Usage:
```rust
let c = Color(255, 0, 0);
println!("{}", c.0);
```

---

## 6. **Unit-like Structs**

```rust
struct Marker;
```

Uses:
- Marker types  
- Zero-sized types  
- Traits without storing data  

---

## 7. **Methods using `impl`**

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

---

## 8. **Associated Functions (Constructors)**

```rust
impl Rectangle {
    fn new(w: u32, h: u32) -> Self {
        Self { width: w, height: h }
    }
}

let r = Rectangle::new(10, 20);
```

---

## 9. **Ownership Rules with Structs**

- Struct fields follow Rust’s ownership rules.
- Moving a struct moves its non-Copy fields.

```rust
let a = User {
    username: String::from("A"),
    age: 20,
    active: true,
};

let b = a; // move
```

---

## 10. **Structs with Lifetimes**

Needed when struct stores references:

```rust
struct Person<'a> {
    name: &'a str,
}
```

---

## 11. **Destructuring Structs**

Full destructuring:

```rust
let User { username, age, active } = u;
```

Partial:

```rust
let User { age, .. } = u;
```

---

# 🔥 Summary Table

| Topic | Concept |
|-------|---------|
| Named Struct | `struct Person { name: String }` |
| Tuple Struct | `struct Point(i32, i32)` |
| Unit Struct | `struct Marker;` |
| Update Syntax | `{ field: new, ..old }` |
| Methods | `fn area(&self)` |
| Associated Function | `fn new() -> Self` |
| Ownership | Struct moves transfer ownership |
| Lifetimes | Needed for reference fields |

---

# 📘 Methods and Associated Functions

## **Methods:**
- Methods are the associated functions where the **first parameter is `self`, `&self`, or `&mut self`**.  
- Methods are called **on the instance** of the type.  
- Methods use the **`instance.func()`** syntax.

## **Associated Functions:**
- Associated functions are functions where **no `self`/`&self`/`&mut self` is passed as the first parameter**.  
- Associated functions are called on the **Type itself**.  
- They use the **`Type::func()`** syntax.

### **NOTE:**  
➡️ *All methods are associated functions, but not all associated functions are methods.*


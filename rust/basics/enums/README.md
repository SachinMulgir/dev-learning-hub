# 🦀 Enums & Option`<T>`{=html}

## 1. What is an Enum?

An enum in Rust defines a custom type with multiple possible variants.\
Each variant may store its own data.

------------------------------------------------------------------------

## 2. Basic Enum

``` rust
enum Direction {
    North,
    South,
    East,
    West,
}
```

### Creating an instance:

``` rust
let d = Direction::North;
```

------------------------------------------------------------------------

## 3. Enums with Associated Values

``` rust
enum IPAddrKind {
    V4(String),
    V6(String),
}
```

### Creating an instance:

``` rust
let home = IPAddrKind::V4(String::from("127.0.0.1"));
let loopback = IPAddrKind::V6(String::from("::1"));
```

------------------------------------------------------------------------

## 4. Reading Associated Values

``` rust
let ip = IPAddrKind::V4(String::from("1.2.3.4"));

match ip {
    IPAddrKind::V4(addr) => println!("IPv4: {}", addr),
    IPAddrKind::V6(addr) => println!("IPv6: {}", addr),
}
```

------------------------------------------------------------------------

## 5. Enums with Multiple Data Types

``` rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

------------------------------------------------------------------------

## 6. Methods on Enums

``` rust
impl Message {
    fn call(&self) {
        println!("Message received!");
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

------------------------------------------------------------------------

## 7. Pattern Matching Essentials

``` rust
match value {
    Variant1 => {},
    Variant2(v) => {},
    Variant3 { field, .. } => {},
}
```

### Using if let:

``` rust
if let IPAddrKind::V4(addr) = ip {
    println!("IPv4: {}", addr);
}
```

------------------------------------------------------------------------

## 8. The Option`<T>`{=html} Enum

``` rust
enum Option<T> {
    Some(T),
    None,
}
```

### Example:

``` rust
let num = Some(10);
let absent: Option<i32> = None;
```

------------------------------------------------------------------------

## 9. Why Option`<T>`{=html} Is Necessary

-   Rust has no null.
-   Option`<T>`{=html} safely represents missing values.
-   Forces handling of both Some and None.
-   Prevents null pointer bugs.

------------------------------------------------------------------------

## 10. How Option`<T>`{=html} Is Memory Efficient

-   Uses niche optimizations.
-   `Option<&T>` stores None as a null pointer.
-   Many types pack the discriminant without extra memory.

------------------------------------------------------------------------

## 11. Unwrapping an Option

``` rust
match val {
    Some(v) => println!("{}", v),
    None => println!("No value"),
}

println!("{}", val.unwrap_or(0));
```

------------------------------------------------------------------------

## 12. Option Combinators

``` rust
val.map(|x| x * 2);
val.unwrap_or(100);
val.and_then(|x| Some(x + 1));
```

------------------------------------------------------------------------

## Summary Table

  Topic                Concept
  -------------------- --------------------------
  Basic Enum           `enum A { X, Y }`
  Associated Values    `V4(String)`
  Pattern Match        `match x { .. }`
  Enum Methods         `impl Enum { fn f(){} }`
  Option`<T>`{=html}   Value or no value
  Some(T)              Value present
  None                 No value present
  Memory Safety        Prevents null bugs
  Zero-Cost            Uses niche optimization

------------------------------------------------------------------------

## Key Takeaways

### Enums

-   Can store multiple data types.
-   Useful for state machines, messages, and configurations.

### Option`<T>`{=html}

-   Rust's safe replacement for null.
-   Enforces exhaustive handling.
-   Memory-efficient and zero-cost.

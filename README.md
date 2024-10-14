# RUST

## VARIABLES IN RUST

### Integer Types:
- **Signed integers:**  
  `i8`, `i16`, `i32`, `i64`, `i128` (default: `i32`)
- **Unsigned integers:**  
  `u8`, `u16`, `u32`, `u64`, `u128`
- **Floating-point numbers:**  
  `f32`, `f64`

#### Example:
```rust
let x: i8 = -23;
let y: u32 = 499;
let z: f32 = 299.829;

print!("x: {}\ny: {}\nz: {}\n", x, y, z);
```

**Note:**  
Integers are easy to understand and faster to change in memory.

---

### Boolean:
- **Type:** `bool`

#### Example:
```rust
let is_male: bool = true;
let is_above_18: bool = true;

if is_male {
    print!("YOU ARE A MALE\n");
}
if is_male && is_above_18 {
    print!("YOU ARE A LEGAL MALE\n");
}
```

**Note:**  
Booleans behave similarly to integers in terms of memory management.

---

### String:
- Strings in Rust are more complex than in other languages.

#### Example:
```rust
let greeting: String = String::from("Hello world!");
print!("{}\n", greeting);

// Pattern matching to safely access a character by index
let char1 = greeting.chars().nth(0);
match char1 {
    Some(c) => println!("Char at index 0 is: {}", c),
    None => println!("NOTHING AT INDEX 0"),
}
```

**Note:**  
Strings are **slower and harder to modify** in memory compared to other types. Rust does not allow direct indexing like other languages to prevent runtime errors. Pattern matching is used for safe access.

---

### Mutability and Loops:
Variables in Rust are immutable by default. To modify a variable's value, you must declare it as `mut`.

#### Example:
```rust
let mut a: i32 = 9;

for i in 0..100 {
    a += i;
}

print!("Value of 'a' after increment: {}\n", a);
```

---

### Pattern Matching:
Pattern matching is used in Rust for handling optional values and other data structures safely.

#### Example:
```rust
let char1 = greeting.chars().nth(0);
match char1 {
    Some(c) => println!("Char at index 0 is: {}", c),
    None => println!("NOTHING AT INDEX 0"),
}
```

---

``` 

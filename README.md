# tuple-for module

This provides a simple macro, `tuple_for!`, which will allow you to write a for loop for any tuple up to 12 elements.
It was written as an experiment and is not meant for actual use.

# Examples

Basic iteration
```rust
let tuple = (1, 2, 3);

let mut sum = 0;
tuple_for!(for t in tuple {
    sum += t;
});

assert_eq!(sum, 6);
```

Completely different types
```rust
struct S {
    apple: u32,
    orange: u32,
    pear: u32,
}

struct T {
    ball: u32,
    rock: u32,
    apple: u32,
}

let mut sum = 0;
let tuple = (
    S {
        apple: 2,
        orange: 20,
        pear: 200,
    },
    T {
        ball: 5,
        rock: 37,
        apple: 40,
    },
);
tuple_for!(for s in tuple {
    sum += s.apple;
});
assert_eq!(sum, 42);
```

Pattern matching
```rust
struct S {
    a: i32,
    b: i32,
}
let mut sum = 0;
let tuple = (S { a: 1, b: 1 }, S { a: 2, b: 3 });
tuple_for!(for S { a, b } in tuple {
    sum += a;
    sum += b;
});

assert_eq!(sum, 7);
```

Tuple reference
```rust
let mut tuple = (1, 2);
tuple_for!(for t in &mut tuple {
    *t += 1;
});
assert_eq!(tuple, (2, 3));
```

Support for break and continue
```rust
{
    let mut sum = 0;
    tuple_for!(for i in (1, 1, 2, 3, 5, 8) {
        if i % 2 == 0 {
            continue;
        }
        sum += i;
    });
    assert_eq!(sum, 10) 
}

{
    let mut sum = 0;
    tuple_for!(for i in (1, 1, 2, 3, 5, 8) {
        sum += 1;
        if i > 2 {
            break;
        }
    });
    assert_eq!(sum, 4)
}
```
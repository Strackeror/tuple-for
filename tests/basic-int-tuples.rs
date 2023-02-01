use tuple_for::tuple_for;

#[test]
fn test_sum_tuple() {
    let tuple = (1, 2, 3);

    let mut sum = 0;
    tuple_for!(for t in tuple {
        sum += t;
    });

    assert_eq!(sum, 6);
}

#[allow(clippy::unnecessary_cast)]
#[test]
fn test_varied_tuple() {
    let tuple = (1u32, 2u64, 3i32);

    let mut sum: u64 = 0;
    tuple_for!(for t in tuple {
        sum += t as u64;
    });

    assert_eq!(sum, 6);
}

#[test]
fn test_pattern_matching() {
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
}

#[test]
fn test_diff_types() {
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
}

#[test]
fn test_as_ref() {
    let tuple = (1, 2);

    let mut sum = 0;
    tuple_for!(for t in &tuple {
        sum += t;
    });

    tuple_for!(for t in &tuple {
        sum += t;
    });
    assert_eq!(sum, 6);
}

#[test]
fn test_as_mut_ref() {
    let mut tuple = (1, 2);
    tuple_for!(for t in &mut tuple {
        *t += 1;
    });

    let mut arr = [1, 2, 3];
    for t in &mut arr {
        *t += 1;
    }

    assert_eq!(tuple, (2, 3));
}

#[test]
fn test_nested_refs() {
    let mut a = 1;
    let mut b = 2;
    let mut c = "to";
    let mut d = "ta";
    {
        let mut tuple = ((&mut a, &mut b), (&mut c, &mut d));
        tuple_for! {for (ta, tb) in &mut tuple {
            std::mem::swap(ta, tb);
        }}
        *tuple.0 .0 = 5;
        *tuple.1 .0 = "toto";
    }
    assert_eq!(b, 5);
    assert_eq!(d, "toto");
}

#[test]
fn test_break() {
    let mut sum = 0;
    tuple_for!(for i in (1, 1, 2, 3, 5, 8) {
        sum += 1;
        if i > 2 {
            break;
        }
    });
    assert_eq!(sum, 4)
}

#[test]
fn test_continue() {
    let mut sum = 0;
    tuple_for!(for i in (1, 1, 2, 3, 5, 8) {
        if i % 2 == 0 {
            continue;
        }
        sum += i;
    });
    assert_eq!(sum, 10)
}

#[test]
fn test_nested_context() {
    let mut sum = 0;
    tuple_for!(for _ in (1, 1, 2, 3) {
        let _ = |_: i32| {
            for i in [0, 1, 2] {
                println!("2");
                if i == 1 {
                    break;
                }
            }
        };

        fn increment(i: &mut i32) {
            for j in [0, 1, 2] {
                if j == 0 {
                    continue;
                }
                *i += 1;
            }
        }

        struct T;
        impl T {
            fn increment(i: &mut i32) {
                for j in [0, 1, 2] {
                    if j == 0 {
                        continue;
                    }
                    *i += 10;
                }
            }
        }

        increment(&mut sum);
        T::increment(&mut sum);
    });

    assert_eq!(sum, 88);
}

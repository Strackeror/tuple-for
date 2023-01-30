use tuple_for::tuple_for;

#[test]
fn test_sum_tuple() {
    let tuple = (1, 2, 3);

    let mut sum = 0;
    tuple_for!(t in tuple => {
      sum += t;
    });

    assert_eq!(sum, 6);
}

#[allow(clippy::unnecessary_cast)]
#[test]
fn test_varied_tuple() {
    let tuple = (1u32, 2u64, 3i32);

    let mut sum: u64 = 0;
    tuple_for!(t in tuple => {
      sum += t as u64;
    });

    assert_eq!(sum, 6);
}

#[test]
fn test_as_ref() {
    let tuple = (1, 2);

    let mut sum = 0;
    tuple_for!(t in &tuple => {
      sum += t;
    });

    tuple_for!(t in &tuple => {
      sum += t;
    });
    assert_eq!(sum, 6);
}

#[test]
fn test_as_mut_ref() {
    let mut tuple = (1, 2);
    tuple_for!(t in &mut tuple => {
      *t += 1;
    });

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
        tuple_for! {(ta, tb) in &mut tuple => {
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
    tuple_for!(i in (1,1,2,3,5) => {
        if i > 2  {
            break;
        }
        sum += i;
    });
    assert_eq!(sum, 4)
}

#[test]
fn test_continue() {
    let mut sum = 0;
    tuple_for!(i in (1,1,2,3,5,8) => {
        if i % 2 == 0  {
            continue;
        }
        sum += i;
    });
    assert_eq!(sum, 10)
}

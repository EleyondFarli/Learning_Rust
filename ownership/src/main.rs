fn main() {
    // Dereferencing a pointer
    let mut x: Box<i32> = Box::new(1);
    let a : i32 = *x;
    *x += 1;


    let r1: &Box<i32> = &x;
    let b: i32 = **r1;

    println!("x={x}\nb={b}");
    {
        let r2: &i32 = &*x;
        let c: i32 = *r2;
        println!("c={c}");
    }
    println!("x={x}");

    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs();      // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs();       // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len();      // implicit reference
    assert_eq!(s_len1, s_len2);
}

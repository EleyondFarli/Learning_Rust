fn main() {
    println!("Hello, world!");
}

// Aliasing and Mutating a Data Structure
//Option 1
// Causes a performance hit for allocating and copying string data
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();

    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}

//Option 2
// Causes a performance hit for allocating the vector to_add
fn add_big_strings2(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
    let to_add: Vec<String> = src
        .iter()
        .filter(|s| s.len() > largest.len())
        .cloned()
        .collect();
    dst.extend(to_add);
}

// Option 3
// Most idiomatic
fn add_big_strings3(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

// Copying vs. Moving Out of a Collection
// if a value does not own heap data, then it can be copied without a move
fn copy_from_vector() {
    let v: Vec<i32> = vec![0, 1, 2];
    let n_ref: &i32 = &v[0];
    let n: i32 = *n_ref;

    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    println!("{s_ref}!");

    let v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v[0].clone();
    s.push('!');
    println!("{s}");

    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.len() == 0);
}

// Mutating different array elements
fn mutating() {
    let mut a = [0, 1, 2, 3];
    let x = &mut a[0];
    *x += 1;
    println!("{a:?}");

    // Problem code
    /*
    let mut a = [0, 1, 2, 3];
    let x = &mut a[0];
    let y = &a[1];
    *x += *y;
     */

    // Solution 1
    let mut a = [0, 1, 2, 3];
    let (x, rest) = a.split_first_mut().unwrap();
    let y = &rest[0];
    *x += *y;

    // Solution 2
    let mut a = [0, 1, 2, 3];
    let x = &mut a[0] as *mut i32;
    let y = &a[1] as *const i32;
    unsafe { *x += *y; } // DO NOT DO THIS unless you know what you're doing!
}

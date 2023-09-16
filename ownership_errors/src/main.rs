fn main() {
    println!("Hello, world!");
}

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
    let to_add: Vec<String> =
        src.iter().filter(|s| s.len() > largest.len()).cloned().collect();
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
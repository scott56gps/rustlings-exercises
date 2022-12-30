// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)

fn main() {
    let mut vec0 = Vec::new();  // vec0 comes into scope

    fill_vec(&mut vec0);  // Because we send a mut reference, vec0 is not taken

    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);  // We could have left vec0 immutable until this point;
                    //  we must make vec0 mutable in order to push to it

    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
}

fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}

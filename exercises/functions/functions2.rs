// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

fn main() {
    call_me(5, 3);
}

fn call_me(num: i32, val: i32) {
    for i in 0..num {
        println!("Ring! Call number {}, with magic number {}", i + 1, i + val);
    }
}

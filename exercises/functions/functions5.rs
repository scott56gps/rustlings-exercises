// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

fn main() {
    let answer = square(7);
    let closure = { square(9) };
    println!("The answer is {}", closure);
}

fn square(num: i32) -> i32 {
    num * num
}

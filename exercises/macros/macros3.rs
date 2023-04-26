// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    macro_rules! greet {
        ($x:expr) => {
            println!("Hello {}!", $x);
        };
    }
}

fn main() {
    my_macro!();
    greet!("Scott");
}

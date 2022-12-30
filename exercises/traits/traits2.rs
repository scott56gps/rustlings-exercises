// traits2.rs
//
// Your task is to implement the trait
// `AppendBar' for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!

trait AppendBar {
    fn append_bar(self) -> Self;
}

// Add your code here
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {   // How does this work?  I thought that the implementing signature must match exactly the trait's (e.g. no 'mut')
        self.push(String::from("Bar"));
        self
    }
}

fn main() {
    let v: Vec<String> = vec![String::from("Hola")];
    v.append_bar();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}

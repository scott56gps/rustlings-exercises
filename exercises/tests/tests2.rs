// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

pub struct Note {
    name: char,
    value: u16,
}

impl Note {
    pub fn new(name: char, value: u16) -> Note {
        if !vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'].contains(&name) {
            panic!("Note name must be between 'a' and 'g', got {}", name);
        }

        Note { name, value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(Note::new('a', 16).name, 'a');
    }
}

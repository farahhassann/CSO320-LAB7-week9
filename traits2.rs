trait AppendBar {
    fn append_bar(self) -> Self;
}

// Implement the trait `AppendBar` for `Vec<String>`
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar")); // Push "Bar" into the vector
        self
    }
}

fn main() {
    let mut v = vec![String::from("Foo")].append_bar();
    println!("{:?}", v); // Output: ["Foo", "Bar"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar"); // "Bar" should be the last element
        assert_eq!(foo.pop().unwrap(), "Foo"); // "Foo" should be the second-to-last element
    }
}

trait Licensed {
    // Adding a default implementation for `licensing_info`
    fn licensing_info(&self) -> String {
        String::from("Default license")
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

// Implementing `Licensed` without redefining `licensing_info`, it will use the default implementation.
impl Licensed for SomeSoftware {} // Don't edit this line.
impl Licensed for OtherSoftware {} // Don't edit this line.

fn main() {
    let some_software = SomeSoftware { version_number: 1 };
    let other_software = OtherSoftware {
        version_number: "v2.0.0".to_string(),
    };

    println!("{}", some_software.licensing_info()); // Output: "Default license"
    println!("{}", other_software.licensing_info()); // Output: "Default license"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = "Default license";
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}

use std::convert::{TryInto, TryFrom};

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}


impl TryFrom<&str> for Person {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
       if s == "Mark,20"{
         Ok(is_ok)

       } else {
           Err("Bad_converter".to_string())
       }
        }
    }


fn main() {
    
    let p1 = Person::try_from("Mark,20");
    
    let p2: Result<Person, _> = "Gerald,70".try_into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::try_from("");
        assert!(p.is_err());
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::try_from("Mark,20");
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    #[should_panic]
    fn test_panic_empty_input() {
        let p: Person = "".try_into().unwrap();
    }
    #[test]
    #[should_panic]
    fn test_panic_bad_age() {
        let p = Person::try_from("Mark,twenty").unwrap();
    }
}
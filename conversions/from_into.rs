#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}


impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}


impl From<&str> for Person {
    fn from(s: &str) -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

fn main() {
    
    let p1 = Person::from("Mark,20");
    
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p2);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        
        let p = Person::from("John,30");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}

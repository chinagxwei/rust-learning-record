#[derive(Debug)]
struct Person {
    name: String
}

impl Person {
    fn new<S: Into<String>>(name: S) -> Person {
        Person { name: name.into() }
    }
}

#[derive(Debug)]
struct Man {
    name: String,
    age: u32,
}

impl From<Person> for Man {
    fn from(person: Person) -> Self {
        Man { name: person.name, age: 10 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_from() {
        let p1 = Person::new("John");
        let p2 = Person::new(String::from("Bili"));
        println!("{:?}", p1);
        let m1: Man = p1.into();
        println!("{:?}", m1);
    }
}

use std::fmt::Display;


pub struct Person {
    pub name: String,
    pub age:  u32,
}

impl Display for Person{

    fn fmt(&self, 
        f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Name: {}, Age {}", self.name, self.age)
    }
}

impl Person {
    pub fn new(name: String, age: u32) -> Person {
        Person {
            name,
            age,
        }
    }

    pub fn display(&self) {
        println!("{}", self);
    }
}
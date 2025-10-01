struct Student {
    name: String,
    major: String,
}

impl Student {
    fn new(name: &str, major: &str) -> Student {
        Student {
            name: name.to_string(),
            major: major.to_string(),
        }
    }

  
    fn set_major(&mut self, new_major: &str) {
        self.major = new_major.to_string();
    }


    fn get_major(&self) -> &str {
        &self.major
    }
}

fn main(){
    let mut student1 = student::new("Alex", "Computer Engineering");

    println!("{}'s major is: {}", student1.name, student1.get_major());

    student1.set_major("Eletrical engineering");
    println!("{}' new major is: {}", student1.name, student1.get_major());
}
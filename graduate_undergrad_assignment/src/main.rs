use std::fmt::Display;

trait ShowInfo {
    fn show_info(&self);
}

struct Undergrad {
    name: String,
    major: String,
    gpa: f32,
}

struct GradStudent {
    name: String,
    major: String,
    gpa: f32,
    thesis: String,
}

impl ShowInfo for Undergrad {
    fn show_info(&self) {
        println!(
            "Undergraduate Student: {}\nMajor: {}\nGPA: {:.2}\n",
            self.name, self.major, self.gpa
        );
    }
}

impl ShowInfo for GradStudent {
    fn show_info(&self) {
        println!(
            "Graduate Student: {}\nMajor: {}\nGPA: {:.2}\nThesis: {}\n",
            self.name, self.major, self.gpa, self.thesis
        );
    }
}

struct MixedEnrollment {
    students: Vec<Box<dyn ShowInfo>>,
}

impl MixedEnrollment {
    fn new() -> Self {
        MixedEnrollment { students: Vec::new() }
    }

    fn add_student<T: ShowInfo + 'static>(&mut self, student: T) {
        self.students.push(Box::new(student));
    }

    fn show_all(&self) {
        for s in &self.students {
            s.show_info();
        }
    }
}

fn main() {
    let u1 = Undergrad {
        name: "Alex Rodriguez".to_string(),
        major: "Computer Engineering".to_string(),
        gpa: 3.6,
    };

    let g1 = GradStudent {
        name: "Maria Lopez".to_string(),
        major: "Electrical Engineering".to_string(),
        gpa: 3.9,
        thesis: "FPGA Optimization for Machine Learning".to_string(),
    };

    let mut enrollment = MixedEnrollment::new();
    enrollment.add_student(u1);
    enrollment.add_student(g1);

    println!("=== All Enrolled Students ===");
    enrollment.show_all();
}

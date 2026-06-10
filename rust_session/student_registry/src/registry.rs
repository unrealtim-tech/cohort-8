use crate::grade::{Grade, Sex};
use crate::student_struct::Student;

#[derive(Debug)]
pub struct Registry {
    pub students: Vec<Student>,
    next_id: u32,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            students: Vec::new(),
            next_id: 0,
        }
    }

    pub fn add(&mut self, name: &str, age: u8, sex: Sex, grade: Grade, score: f32) {
        let id = self.next_id;
        let student = Student::new(id, name.to_string(), age, sex, grade, score);
        println!("Added: {} (ID {})", student.name, student.id);
        self.students.push(student);
        self.next_id += 1;
    }

    pub fn list_all(&self) {
        if self.students.is_empty() {
            println!("  (no students enrolled yet)");
            return;
        }
        println!(
            "  {:>5}  {:<20}  {:<6}  {:<10}  {}",
            "ID", "Name", "Age", "Grade", "Score"
        );
        println!("  {}", "-".repeat(55));
        for student in &self.students {
            println!(
                "  {:>5}  {:<20}  {:>6}  {:<10}  {:.1}",
                student.id,
                student.name,
                student.age,
                student.grade.as_str(),
                student.score,
            );
        }
    }

    pub fn find_student_by_id(&self, id: u32) -> Option<&Student> {
        match self.students.iter().find(|s| s.id == id) {
            Some(s) => Some(s),
            None => None,
        }
    }

    pub fn update(&mut self, id: u32, student: Student) {
        let Some(s) = self.students.iter().find(|s| s.id == id) else {
            println!("Student with ID {} not found", id);
            return;
        };

        if student.name == s.name
            && student.age == s.age
            && student.grade == s.grade
            && student.score == s.score
        {
            println!("No updates found, cannot make update.");
            return;
        }

        match self.students.iter_mut().find(|s| s.id == id) {
            Some(s) => {
                *s = student;
                println!("Student with ID {} updated", id);
            }
            None => println!("Student with ID {} not found", id),
        }
    }

    pub fn delete(&mut self, id: u32) {
        let position = self.students.iter().position(|s| s.id == id);

        match position {
            Some(position_index) => {
                self.students.remove(position_index);
                println!("Student with ID {} has been deleted", id);
            }
            None => println!("Student with ID {} not found", id),
        }
    }
}

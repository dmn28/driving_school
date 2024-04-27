use chrono::{NaiveDate, Utc};
use crate::models::student::Student;

//TODO TESTS :(
pub struct DrivingSchool {
    pub students: Vec<Student>,
}

impl DrivingSchool {
    pub fn new() -> Self {
        Self {
            students: Vec::new()
        }
    }

    pub fn add_student(&mut self, student: Student) {
        self.students.push(student)
    }

    // pub fn list_all_students(&self) -> Vec<&Student>{
    //     self.students.iter().cloned().collect()
    // }

    pub fn students_needing_eye_test(&self) -> Vec<&Student> {
        self.students.iter().filter(|student| student.passed_eye_test == true).collect()
    }

    pub fn students_ready_for_exam(&self) -> Vec<&Student> {
        self.students.iter().filter(|student| student.minimum_lessons_remaining() == 0).collect()
    }

    pub fn students_with_upcoming_exam(&self, in_days: u8) -> Vec<&Student> {
        let naive_in_days = Utc::now().naive_utc().checked_add_days(chrono::Days::new(in_days as u64)).unwrap().date();
        self.students
            .iter()
            .filter(|student| student.exam_date.is_some())
            .filter(|student| student.exam_date.unwrap() <= naive_in_days)
            .collect()
    }

    pub fn clean_up_students(&mut self) {
        self.students.retain(|student| !student.passed_exam);
        for student in &mut self.students {
            student.name.clear();
            student.date_of_birth = NaiveDate::from_ymd_opt(0, 1, 1).unwrap();
        }
    }

    pub fn print(&self) {
        println!("pozdrav iz drivingschool");
    }
}
use std::fmt::{Display, Formatter};
use chrono::{Datelike, NaiveDate, Utc};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct StudentInput {
    pub date_of_birth: chrono::NaiveDate,
    pub has_id: bool,
    pub passed_eye_test: bool,
    pub name: String,
}

//TODO move CarType out of student

#[derive(Serialize, Deserialize)]
pub struct Student {
    pub name: String,
    pub date_of_birth: chrono::NaiveDate,
    pub has_id: bool,
    pub passed_eye_test: bool,
    pub lessons_completed: u16,
    pub car_type: CarType,
    pub exam_date: Option<chrono::NaiveDate>,
    pub passed_exam: bool,
}

impl Student {
    pub fn new(name: String,
               date_of_birth: chrono::NaiveDate,
               has_id: bool,
               passed_eye_test: bool,
    ) -> Result<Self> {
        if name.is_empty() {
            return Err(anyhow!("Name cannot be empty"));
        }

        if !has_id {
            return Err(anyhow!("student must have an id"));
        }

        let student = Student {
            name,
            date_of_birth,
            has_id,
            passed_eye_test,
            lessons_completed: 0,
            car_type: CarType::Manual,
            exam_date: None,
            passed_exam: false,
        };

        if student.can_start_driving_lessons() == true {
            Ok(student)
        } else {
            Err(anyhow!("student not eligible to start driving lessons"))
        }
    }

    pub fn with_car_type(mut self, car_type: CarType) -> Self {
        self.car_type = car_type;
        self
    }

    pub fn can_start_driving_lessons(&self) -> bool {
        self.is_seventeen_in_six_months() && self.has_id && self.passed_eye_test
    }

    pub fn complete_lesson(mut self) -> Self {
        self.lessons_completed = self.lessons_completed + 1;
        self
    }

    //TODO add tests
    pub fn minimum_lessons_remaining(&self) -> u8 {
        match self.car_type {
            CarType::Manual => 25 - self.lessons_completed as u8,
            CarType::Automatic => 20 - self.lessons_completed as u8,
        }
    }

    pub fn set_exam_date(&mut self, exam_date: NaiveDate) -> Result<()> {
        let check_date = NaiveDate::from_ymd_opt(exam_date.year(), exam_date.month(), exam_date.day());

        if check_date.is_some() {
            self.exam_date = Some(exam_date);
            Ok(())
        } else {
            Err(anyhow!("invalid exam date"))
        }
    }

    //TODO add tests
    pub fn is_seventeen_in_six_months(&self) -> bool {
        let naive_date_time_now = Utc::now().naive_utc();

        let year_now = naive_date_time_now.year();
        let month_now = naive_date_time_now.month();
        let _day_now = naive_date_time_now.day();

        let year_difference = year_now - self.date_of_birth.year();

        if year_difference < 16 {
            false
        } else {
            let month_difference = month_now - self.date_of_birth.month();
            if month_difference > 6 {
                false
            } else {
                true
            }
        }
    }
}

impl Display for Student {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}; Car type: {}; Lessons completed: {}; Lessons remaining: {}",
               self.name,
               self.car_type,
               self.lessons_completed,
               self.minimum_lessons_remaining())
    }
}

//TODO move CarType out of student
#[derive(Serialize, Deserialize)]
pub enum CarType {
    Manual,
    Automatic,
}

impl Display for CarType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Automatic => write!(f, "Automatic"),
            Self::Manual => write!(f, "Manual")
        }
    }
}
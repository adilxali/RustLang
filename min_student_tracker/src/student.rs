// src/student.rs
#[derive(Debug)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub grades: Vec<u8>,
}

impl Student {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            grades: Vec::new(),
        }
    }

    pub fn add_grade(&mut self, grade: u8) {
        self.grades.push(grade);
    }

    pub fn average(&self) -> f32 {
        if self.grades.is_empty() {
            0.0
        } else {
            let sum: u32 = self.grades.iter().map(|g| *g as u32).sum();
            sum as f32 / self.grades.len() as f32
        }
    }
}

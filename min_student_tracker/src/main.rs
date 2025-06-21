// src/main.rs
mod student;
use std::collections::HashMap;
use crate::student::Student;
use std::io::{self, Write};

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn main() {
    let mut students: HashMap<u32, Student> = HashMap::new();

    loop {
        println!("\n🎓 Student Tracker Menu");
        println!("1. Add Student");
        println!("2. Add Grade");
        println!("3. View Average");
        println!("4. Show All Students");
        println!("5. Exit");

        let choice = read_input("Enter your choice: ");
        match choice.as_str() {
            "1" => {
                let id: u32 = read_input("Enter student ID: ").parse().unwrap_or(0);
                let name = read_input("Enter student name: ");
                let student = Student::new(id, &name);
                students.insert(id, student);
                println!("✅ Student added.");
            }
            "2" => {
                let id: u32 = read_input("Enter student ID: ").parse().unwrap_or(0);
                if let Some(student) = students.get_mut(&id) {
                    let grade: u8 = read_input("Enter grade (0–100): ").parse().unwrap_or(0);
                    student.add_grade(grade);
                    println!("✅ Grade added.");
                } else {
                    println!("❌ Student not found.");
                }
            }
            "3" => {
                let id: u32 = read_input("Enter student ID: ").parse().unwrap_or(0);
                if let Some(student) = students.get(&id) {
                    println!(
                        "{}'s average grade: {:.2}",
                        student.name,
                        student.average()
                    );
                } else {
                    println!("❌ Student not found.");
                }
            }
            "4" => {
                for (id, student) in &students {
                    println!("\nID: {}, Name: {}, Grades: {:?}, Avg: {:.2}",
                             id, student.name, student.grades, student.average());
                }
            }
            "5" => {
                println!("👋 Exiting...");
                break;
            }
            _ => println!("⚠️ Invalid choice!"),
        }
    }
}

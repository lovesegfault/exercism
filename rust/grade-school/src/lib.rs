use std::collections::HashMap;

#[derive(Default)]
pub struct School {
    roster: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.roster
            .entry(grade)
            .and_modify(|v| v.push(student.to_string()))
            .or_insert_with(|| vec![student.to_string()]);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.roster.keys().copied().collect();
        grades.sort_unstable();
        grades
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students: Vec<String> = self
            .roster
            .get(&grade)
            .cloned()
            .unwrap_or_else(|| Vec::new());
        students.sort_unstable();
        students
    }
}

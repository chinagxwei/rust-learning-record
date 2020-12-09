#[derive(Clone)]
struct StudentVO {
    name: &'static str,
    roll_no: usize,
}

impl StudentVO {
    pub fn new(name: &'static str, roll_no: usize) -> Self {
        StudentVO { name, roll_no }
    }
}

impl StudentVO {
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn roll_no(&self) -> usize {
        self.roll_no
    }

    pub fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }
    pub fn set_roll_no(&mut self, roll_no: usize) {
        self.roll_no = roll_no;
    }
}

struct StudentBO {
    students: Vec<StudentVO>
}

impl StudentBO {
    pub fn new() -> Self {
        StudentBO {
            students: vec![
                StudentVO::new("Robert", 0),
                StudentVO::new("John", 1)
            ]
        }
    }
}

impl StudentBO {
    fn get_student(&self, roll_no: usize) -> StudentVO {
        self.students.get(roll_no).unwrap().clone()
    }

    fn delete_student(&mut self, student: StudentVO) {
        self.students.remove(student.roll_no);
        println!("Student: Roll No {}, deleted from database", student.roll_no())
    }
    fn update_student(&mut self, student: StudentVO) {
        self.students
            .get_mut(student.roll_no())
            .unwrap()
            .set_name(student.name());
        println!("Student: Roll No {}, updated in the database", student.roll_no())
    }
    pub fn get_students(&self) -> &Vec<StudentVO> {
        &self.students
    }

    fn set_student_name(&mut self, i: usize, name: &'static str) {
        let mut student = self.get_student(i);
        student.set_name(name);
        self.update_student(student);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_object() {
        let mut student_bo = StudentBO::new();

        student_bo.get_students()
            .iter()
            .for_each(|x| println!("Student: [RollNo : {}, Name : {} ]", x.roll_no, x.name));

        student_bo.set_student_name(0, "Michael");
        let student = student_bo.get_student(0);
        println!("Student: [RollNo : {}, Name : {} ]", student.roll_no, student.name)
    }
}

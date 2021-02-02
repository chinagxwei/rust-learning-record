//!
//! 数据访问对象模式（Data Access Object Pattern）或 DAO 模式用于把低级的数据访问 API 或操作从高级的业务服务中分离出来。
//! 以下是数据访问对象模式的参与者。
//! 数据访问对象接口（Data Access Object Interface） - 该接口定义了在一个模型对象上要执行的标准操作。
//! 数据访问对象实体类（Data Access Object concrete class） - 该类实现了上述的接口。该类负责从数据源获取数据，数据源可以是数据库，也可以是 xml，或者是其他的存储机制。
//! 模型对象/数值对象（Model Object/Value Object） - 该对象是简单的 POJO，包含了 get/set 方法来存储通过使用 DAO 类检索到的数据。
//!

trait StudentDao {
    fn get_all_students(&self) -> Vec<Student>;
    fn get_student(&self, index: usize) -> Student;
    fn update_student(&mut self, student: Student);
    fn delete_student(&mut self, student: Student);
}

#[derive(Clone, Debug)]
struct Student {
    name: String,
    roll_no: usize,
}

impl Student {
    fn new(name: String, roll_no: usize) -> Student {
        Student { name, roll_no }
    }
}

impl Student {
    fn get_name(&self) -> String {
        self.name.to_owned()
    }

    fn get_roll_no(&self) -> usize {
        self.roll_no
    }

    fn set_name(&mut self, name: String) {
        self.name = name
    }

    fn set_roll_no(&mut self, roll_no: usize) {
        self.roll_no = roll_no
    }
}

struct StudentDaoImpl {
    students: Vec<Student>
}

impl StudentDaoImpl {
    fn new() -> StudentDaoImpl {
        StudentDaoImpl {
            students: vec![
                Student::new(String::from("Robert"), 0),
                Student::new(String::from("John"), 1)
            ]
        }
    }
}

impl StudentDao for StudentDaoImpl {
    fn get_all_students(&self) -> Vec<Student> {
        self.students.clone()
    }

    fn get_student(&self, index: usize) -> Student {
        self.students.get(index).unwrap().clone()
    }

    fn update_student(&mut self, student: Student) {
        self.students
            .get_mut(student.get_roll_no())
            .unwrap()
            .set_name(student.get_name());
        println!("Student: Roll No {}, updated in the database", student.get_roll_no())
    }

    fn delete_student(&mut self, student: Student) {
        self.students
            .remove(student.get_roll_no());
        println!("Student: Roll No {}, deleted from database", student.get_roll_no())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_access() {
        let mut student_dao = StudentDaoImpl::new();

        student_dao.get_all_students()
            .iter()
            .for_each(|x| println!("Student: [RollNo : {}, Name : {} ]", x.get_roll_no(), x.get_name()));

        let mut student = student_dao.get_student(0);
        student.set_name(String::from("Michael"));
        student_dao.update_student(student);

        let student = student_dao.get_student(0);
        println!("Student: [RollNo : {}, Name : {} ]", student.get_roll_no(), student.get_name())
    }
}

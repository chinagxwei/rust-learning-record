//!
//! MVC 模式代表 Model-View-Controller（模型-视图-控制器） 模式。这种模式用于应用程序的分层开发。
//!
//! Model（模型） - 模型代表一个存取数据的对象或 POJO。它也可以带有逻辑，在数据变化时更新控制器。
//! View（视图） - 视图代表模型包含的数据的可视化。
//! Controller（控制器） - 控制器作用于模型和视图上。它控制数据流向模型对象，并在数据变化时更新视图。它使视图与模型分离开。
//!

struct Student {
    name: Option<&'static str>,
    roll_no: Option<&'static str>,
}

impl Student {
    fn new() -> Student {
        Student { name: None, roll_no: None }
    }
}

impl Student {
    pub fn get_name(&self) -> Option<&'static str> {
        self.name
    }
    pub fn get_roll_no(&self) -> Option<&'static str> {
        self.roll_no
    }

    pub fn set_name(&mut self, name: Option<&'static str>) {
        self.name = name;
    }
    pub fn set_roll_no(&mut self, roll_no: Option<&'static str>) {
        self.roll_no = roll_no;
    }
}

struct StudentView;

impl StudentView {
    fn new() -> StudentView {
        StudentView
    }
}

impl StudentView {
    fn render(&self, student_name: &str, student_roll_no: &str) {
        println!("Student: ");
        println!("Name: {}", student_name);
        println!("Roll No: {}", student_roll_no);
    }
}

struct StudentController {
    model: Student,
    view: StudentView,
}

impl StudentController {
    fn new(model: Student, view: StudentView) -> StudentController {
        StudentController { model, view }
    }
}

impl StudentController {
    pub fn get_student_name(&self) -> &'static str {
        &self.model.get_name().unwrap()
    }
    pub fn get_student_roll_no(&self) -> &'static str {
        &self.model.get_roll_no().unwrap()
    }
    pub fn set_student_name(&mut self, student_name: &'static str) {
        self.model.set_name(Some(student_name));
    }
    pub fn set_student_roll_no(&mut self, student_roll_no: &'static str) {
        self.model.set_roll_no(Some(student_roll_no));
    }

    fn render_view(&self) {
        self.view.render(self.model.get_name().unwrap(), self.model.get_roll_no().unwrap());
    }
}

fn retrieve_student_from_database() -> Student {
    let mut student = Student::new();
    student.set_name(Some("Robert"));
    student.set_roll_no(Some("10"));
    student
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_view_controller() {
        let model = retrieve_student_from_database();

        let view = StudentView::new();

        let mut controller = StudentController::new(model, view);

        controller.render_view();

        controller.set_student_name("John");

        controller.render_view();
    }
}

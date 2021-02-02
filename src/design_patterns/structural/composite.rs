//! 组合模式（Composite Pattern），又叫部分整体模式，是用于把一组相似的对象当作一个单一的对象。
//! 组合模式依据树形结构来组合对象，用来表示部分以及整体层次。
//! 这种类型的设计模式属于结构型模式，它创建了对象组的树形结构。
//!
//! 这种模式创建了一个包含自己对象组的类。
//! 该类提供了修改相同对象组的方式。
//!
//! 我们通过下面的实例来演示组合模式的用法。
//! 实例演示了一个组织中员工的层次结构。
//!

use std::slice::Iter;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Employee {
    name: &'static str,
    dept: &'static str,
    salary: i32,
    subordinates: Vec<Employee>,
}

impl Employee {
    fn new(name: &'static str, dept: &'static str, salary: i32) -> Employee {
        Employee {
            name,
            dept,
            salary,
            subordinates: vec![],
        }
    }
}

impl Employee {
    fn add(&mut self, employee: Employee) {
        self.subordinates.push(employee);
    }

    fn remove(&mut self, employee: &Employee) {
        let pos = self.subordinates.iter().position(|x| *x == *employee).unwrap();
        self.subordinates.remove(pos);
    }

    fn get_subordinates(&self) -> Iter<'_, Employee> {
        self.subordinates.iter()
    }

    fn as_str(&self) {
        println!("Employee :[ Name : {}, dept : {}, salary : {} ]", self.name, self.dept, self.salary);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composite() {
        let mut ceo = Employee::new("John", "CEO", 30000);

        let mut head_sales = Employee::new("Robert", "Head Sales", 20000);

        let mut head_marketing = Employee::new("Michel", "Head Marketing", 20000);

        let clerk1 = Employee::new("Laura", "Marketing", 10000);
        let clerk2 = Employee::new("Bob", "Marketing", 10000);

        let sales_executive1 = Employee::new("Richard", "Sales", 10000);
        let sales_executive2 = Employee::new("Rob", "Sales", 10000);

        head_sales.add(sales_executive1);
        head_sales.add(sales_executive2);

        head_marketing.add(clerk1);
        head_marketing.add(clerk2);

        ceo.add(head_sales);
        ceo.add(head_marketing);

        ceo.as_str();

        for i in ceo.get_subordinates() {
            i.as_str();
            for j in i.get_subordinates() {
                j.as_str();
            }
        }
    }
}



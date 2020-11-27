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



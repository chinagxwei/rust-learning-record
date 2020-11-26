use std::thread::sleep;
use std::rc::Rc;

trait Criteria {
    fn meet_criteria(&self, person: &Vec<Person>) -> Vec<Person>;
}

#[derive(Debug, Clone)]
struct Person {
    name: String,
    gender: String,
    marital_status: String,
}

impl Person {
    fn new(name: String, gender: String, marital_status: String) -> Person {
        Person {
            name,
            gender,
            marital_status,
        }
    }
}

impl Person {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_gender(&self) -> &String {
        &self.gender
    }

    fn get_marital_status(&self) -> &String {
        &self.marital_status
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name() &&
            self.get_gender() == other.get_gender() &&
            self.get_marital_status() == other.get_marital_status()
    }
}

struct CriteriaMale;

impl CriteriaMale {
    fn new() -> CriteriaMale {
        CriteriaMale
    }
}

impl Criteria for CriteriaMale {
    fn meet_criteria(&self, person: &Vec<Person>) -> Vec<Person> {
        person
            .iter()
            .filter(|x| { x.get_gender() == "Male" })
            .cloned()
            .collect::<Vec<Person>>()
    }
}


struct CriteriaFemale;

impl CriteriaFemale {
    fn new() -> CriteriaFemale {
        CriteriaFemale
    }
}

impl Criteria for CriteriaFemale {
    fn meet_criteria(&self, person: &Vec<Person>) -> Vec<Person> {
        person
            .iter()
            .filter(|x| { x.get_gender() == "Female" })
            .cloned()
            .collect::<Vec<Person>>()
    }
}

#[derive(Clone)]
struct CriteriaSingle;

impl CriteriaSingle {
    fn new() -> CriteriaSingle {
        CriteriaSingle
    }
}

impl Criteria for CriteriaSingle {
    fn meet_criteria(&self, person: &Vec<Person>) -> Vec<Person> {
        person
            .iter()
            .filter(|x| { x.get_marital_status() == "Single" })
            .cloned()
            .collect::<Vec<Person>>()
    }
}

struct AndCriteria<'a, 'b> {
    criteria: Box<&'a dyn Criteria>,
    other_criteria: Box<&'b dyn Criteria>,
}

impl<'a, 'b> AndCriteria<'a, 'b> {
    fn new(criteria: Box<&'a dyn Criteria>, other_criteria: Box<&'b dyn Criteria>) -> AndCriteria<'a, 'b> {
        AndCriteria { criteria, other_criteria }
    }
}

impl Criteria for AndCriteria<'_, '_> {
    fn meet_criteria(&self, person: &Vec<Person>) -> Vec<Person> {
        let first = self.criteria.meet_criteria(person);
        self.other_criteria.meet_criteria(&first)
    }
}

struct OrCriteria<'a, 'b> {
    criteria: Box<&'a dyn Criteria>,
    other_criteria: Box<&'b dyn Criteria>,
}

impl<'a, 'b> OrCriteria<'a, 'b> {
    fn new(criteria: Box<&'a dyn Criteria>, other_criteria: Box<&'b dyn Criteria>) -> OrCriteria<'a, 'b> {
        OrCriteria { criteria, other_criteria }
    }
}

impl Criteria for OrCriteria<'_, '_> {
    fn meet_criteria(&self, person: &Vec<Person>) -> Vec<Person> {
        let mut first = self.criteria.meet_criteria(person);
        let second = self.other_criteria.meet_criteria(person);

        for p in second {
            if !first.contains(&p) {
                first.push(p)
            }
        }

        first
    }
}

fn print(person: &Vec<Person>) {
    for person in person {
        println!(
            "Person : [ Name : {}, Gender : {}, Marital Status : {} ]",
            person.get_name(),
            person.get_gender(),
            person.get_marital_status()
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        let mut persons = vec![
            Person::new(
                String::from("Robert"),
                String::from("Male"),
                String::from("Single"),
            ),
            Person::new(
                String::from("John"),
                String::from("Male"),
                String::from("Married"),
            ),
            Person::new(
                String::from("Laura"),
                String::from("Female"),
                String::from("Married"),
            ),
            Person::new(
                String::from("Diana"),
                String::from("Female"),
                String::from("Single"),
            ),
            Person::new(
                String::from("Mike"),
                String::from("Male"),
                String::from("Single"),
            ),
            Person::new(
                String::from("Bobby"),
                String::from("Male"),
                String::from("Single"),
            ),
        ];

        let male = CriteriaMale::new();
        let female = CriteriaFemale::new();
        let single = CriteriaSingle::new();

        let single_male = AndCriteria::new(Box::new(&single), Box::new(&male));
        let single_or_female = OrCriteria::new(Box::new(&single), Box::new(&single));

        println!("Males: ");
        print(&male.meet_criteria(&persons));

        println!("\nFemales: ");
        print(&female.meet_criteria(&persons));

        println!("\nSingle Males: ");
        print(&single_male.meet_criteria(&persons));

        println!("\nSingle Or Females: ");
        print(&single_or_female.meet_criteria(&persons));
    }
}





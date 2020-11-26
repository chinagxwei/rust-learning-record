use std::cell::RefCell;

///
/// 迭代器 Iterator trait
///
trait Iterator<T> {
    fn has_next(&self) -> bool;
    fn next(&self) -> Option<T>;
}

///
/// 容器trait
///
trait Container<T> {
    fn get_iterator(&self) -> Box<dyn Iterator<T> + '_>;
}

///
/// 迭代器结构
///
struct NameIterator<'repository_lifetime> {
    repository: &'repository_lifetime NameRepository,
    index: RefCell<usize>,
}

///
/// 迭代器结构静态方法实现
///
impl<'repository_lifetime> NameIterator<'repository_lifetime> {
    fn new(repository: &'repository_lifetime NameRepository) -> NameIterator<'repository_lifetime> {
        NameIterator { repository, index: RefCell::new(0) }
    }
}

///
/// 迭代器 Iterator trait 动态实现
///
impl Iterator<String> for NameIterator<'_> {
    fn has_next(&self) -> bool {
        *self.index.borrow() < self.repository.names.len()
    }

    fn next(&self) -> Option<String> {
        if self.has_next() {
            let res = Some(self.repository.names.get(*self.index.borrow()).unwrap().to_owned());
            *self.index.borrow_mut() += 1;
            return res;
        }
        None
    }
}

///
/// 容器结构
///
struct NameRepository {
    names: Vec<String>
}

///
/// 容器动态实现
///
impl NameRepository {
    fn new(names: Vec<String>) -> NameRepository {
        NameRepository { names }
    }
}

///
/// 容器 Container trait 动态实现
///
impl Container<String> for NameRepository {
    fn get_iterator(&self) -> Box<dyn Iterator<String> + '_> {
        Box::new(NameIterator::new(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let nr = NameRepository::new(
            vec![
                String::from("Robert"),
                String::from("John"),
                String::from("Julie"),
                String::from("Lora")
            ]
        );

        let i = nr.get_iterator();
        while i.has_next() {
            let name = i.next().unwrap();
            println!("Name : {}", name);
        }
    }
}


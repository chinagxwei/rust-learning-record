//!
//! 原型模式（Prototype Pattern）是用于创建重复的对象，同时又能保证性能。
//! 这种类型的设计模式属于创建型模式，它提供了一种创建对象的最佳方式。
//!
//! 这种模式是实现了一个原型接口，该接口用于创建当前对象的克隆。
//! 当直接创建对象的代价比较大时，则采用这种模式。
//! 例如，一个对象需要在一个高代价的数据库操作之后被创建。
//! 我们可以缓存该对象，在下一个请求时返回它的克隆，在需要的时候更新数据库，以此来减少数据库调用。
//!

// use std::borrow::Borrow;

trait Inode {
    fn print(&self, indentation: String);
    fn clone_self(&self) -> Box<dyn Inode>;
}

struct File {
    name: String
}


impl File {
    fn new<S: Into<String>>(name: S) -> File {
        File { name: name.into() }
    }
}

impl Inode for File {
    fn print(&self, indentation: String) {
        println!("{}{}", indentation, self.name)
    }

    fn clone_self(&self) -> Box<dyn Inode> {
        let mut name = self.name.clone();
        name.push_str("_clone");
        Box::new(File::new(name))
    }
}

struct Folder {
    name: String,
    children: Vec<Box<dyn Inode>>,
}

impl Folder {
    fn new<S: Into<String>>(name: S, children: Vec<Box<dyn Inode>>) -> Folder {
        Folder { name: name.into(), children }
    }
}

impl Inode for Folder {
    fn print(&self, indentation: String) {
        println!("{}{}", indentation, self.name);
        self.children
            .iter()
            .for_each(|x| x.print(format!("{}{}", indentation, indentation)))
    }

    fn clone_self(&self) -> Box<dyn Inode> {
        let mut clone_children = vec![];
        self.children.iter()
            .for_each(|x| clone_children.push(x.clone_self()));
        let mut name = self.name.clone();
        name.push_str("_clone");
        Box::new(Folder::new(name, clone_children))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prototype() {
        let file_1 = File::new("File_1");
        let file_2 = File::new("File_2");
        let file_3 = File::new("File_3");

        let folder_1 = Folder::new(
            "Folder_1",
            vec![Box::new(file_1)],
        );

        let folder_2 = Folder::new(
            "Folder_2",
            vec![Box::new(folder_1), Box::new(file_2), Box::new(file_3)],
        );

        println!("\nPrinting hierarchy for Folder2");
        folder_2.print(String::from("  "));

        let clone_folder = folder_2.clone_self();
        println!("\nPrinting hierarchy for clone Folder");
        clone_folder.print(String::from("  "));
    }
}

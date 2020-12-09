use std::borrow::Borrow;

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

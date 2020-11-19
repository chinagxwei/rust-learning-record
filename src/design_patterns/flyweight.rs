use std::error::Error;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::SystemTime;

#[derive(Debug)]
struct Book {
    id: u32,
    title: String,
    author: String,
    is_bind: &'static str,
}

impl Book {
    pub fn new(id: u32, title: String, author: String, is_bind: &'static str) -> Book {
        Book {
            id,
            title,
            author,
            is_bind,
        }
    }
}

#[derive(Debug)]
struct BookFactory {
    existing_books: HashMap<String, Rc<Book>>
}

impl BookFactory {
    pub fn new() -> BookFactory {
        let map: HashMap<String, Rc<Book>> = HashMap::new();
        BookFactory { existing_books: map }
    }

    pub fn create_book(&mut self, id: u32, title: String, author: String, is_bind: &'static str) -> Option<Rc<Book>> {
        let rbook = self.existing_books
            .entry(String::from(is_bind))
            .or_insert(Rc::new(Book::new(id, title, author, is_bind)));
        Some(rbook.clone())
    }
}

#[derive(Debug)]
struct BookRecord {
    book: Rc<Book>,
    checkout_member: String,
    checkout_date: u64,
    due_return_date: u64,
    availability: u64,
}

impl BookRecord {
    pub fn new(book: Rc<Book>) -> BookRecord {
        BookRecord {
            book,
            checkout_member: "".to_string(),
            checkout_date: 0,
            due_return_date: 0,
            availability: 0,
        }
    }
}

#[derive(Debug)]
struct BookRecordManager {
    book_factory: BookFactory,
    book_record_database: HashMap<String, BookRecord>,
}

impl BookRecordManager {
    pub fn new(factory: BookFactory) -> BookRecordManager {
        let hm: HashMap<String, BookRecord> = HashMap::new();
        BookRecordManager { book_factory: factory, book_record_database: hm }
    }

    ///
    /// 添加书籍借出记录
    ///
    fn add_book_record(&mut self, id: u32, title: String, author: String, is_bind: &'static str) {
        let book = self.book_factory.create_book(id, title, author, is_bind).unwrap();
        self.book_record_database.insert(String::from(is_bind), BookRecord::new(book));
    }

    ///
    /// 书籍借出状态更新
    ///
    fn update_checkout_status(&mut self, bind_id: String, checkout_member: String, new_status: u64, checkout_date: u64, new_return_date: u64) {
        let record = self.book_record_database.get_mut(bind_id.as_str()).unwrap();
        record.availability = new_status;
        record.checkout_date = checkout_date;
        record.checkout_member = checkout_member;
        record.due_return_date = new_return_date;
    }

    ///
    /// 书籍续借
    ///
    fn extend_checkout_status(&mut self, bind_id: String, new_return_date: u64) {
        let record = self.book_record_database
            .get_mut(bind_id.as_str())
            .unwrap();
        let now_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        record.due_return_date = now_time + new_return_date;
    }

    ///
    /// 判断书籍借出是否超期
    ///
    fn is_past_due(&self, bind_id: String) -> bool {
        let record = self.book_record_database.get(bind_id.as_str()).unwrap();
        let now_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now_time > record.due_return_date
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_flyweight() {
        let bf = BookFactory::new();

        let mut brm = BookRecordManager::new(bf);

        brm.add_book_record(1, "Rust 编程".to_string(), "alex".to_string(), "1");

        brm.extend_checkout_status("1".to_string(), (86400 as u64));

//    println!("{:?}", brm);

        assert_eq!(brm.is_past_due("1".to_string()), false);
    }
}

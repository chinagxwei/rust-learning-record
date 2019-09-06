///
/// 核算特性
///
trait Accounting {
    fn cost(&self) -> u32;
}

///
/// 书籍结构
///
struct MyBook {
    price: u32
}

impl Accounting for MyBook {
    fn cost(&self) -> u32 {
        self.price
    }
}

impl MyBook {
    fn new() -> Box<MyBook> {
        Box::new(MyBook { price: 1000 })
    }
}

///
/// 存储结构
///
struct Memory {
    book: Box<dyn Accounting>
}

impl Memory {
    fn new(book: Box<dyn Accounting>) -> Box<Memory> {
        Box::new(Memory { book })
    }
}

impl Accounting for Memory {
    fn cost(&self) -> u32 {
        self.book.cost() + 75
    }
}

///
/// 蓝光结构
///
struct BlurayDrive {
    book: Box<dyn Accounting>
}

impl BlurayDrive {
    fn new(book: Box<dyn Accounting>) -> Box<BlurayDrive> {
        Box::new(BlurayDrive { book })
    }
}

impl Accounting for BlurayDrive {
    fn cost(&self) -> u32 {
        self.book.cost() + 300
    }
}

///
/// 保险结构
///
struct Insurance {
    book: Box<dyn Accounting>
}

impl Insurance {
    fn new(book: Box<dyn Accounting>) -> Box<Insurance> {
        Box::new(Insurance { book })
    }
}

impl Accounting for Insurance {
    fn cost(&self) -> u32 {
        self.book.cost() + 250
    }
}


fn main() {
    let book = MyBook::new();
    let memory = Insurance::new(
        BlurayDrive::new(
            Memory::new(
                book
            )
        )
    );

    println!("{}", memory.cost());
}

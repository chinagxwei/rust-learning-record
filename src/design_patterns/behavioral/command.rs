//!
//! 命令模式（Command Pattern）是一种数据驱动的设计模式，它属于行为型模式。
//! 请求以命令的形式包裹在对象中，并传给调用对象。
//! 调用对象寻找可以处理该命令的合适的对象，并把该命令传给相应的对象，该对象执行命令。
//!

trait Order {
    fn execute(&self);
}

struct Stock {
    name: String,
    quantity: i32,
}

impl Stock {
    fn new(name: String, quantity: i32) -> Stock {
        Stock { name, quantity }
    }
}

impl Stock {
    fn buy(&self) {
        println!("Stock [ Name: {} Quantity: {} ] bought", self.name, self.quantity)
    }

    fn sell(&self) {
        println!("Stock [ Name: {} Quantity: {} ] sold", self.name, self.quantity)
    }
}

struct BuyStock<'a> {
    stock: &'a Stock
}

impl<'a> BuyStock<'a> {
    fn new(stock: &'a Stock) -> BuyStock {
        BuyStock { stock }
    }
}

impl Order for BuyStock<'_> {
    fn execute(&self) {
        self.stock.buy();
    }
}

struct SellStock<'a> {
    stock: &'a Stock
}

impl<'a> SellStock<'a> {
    fn new(stock: &'a Stock) -> SellStock {
        SellStock { stock }
    }
}

impl Order for SellStock<'_> {
    fn execute(&self) {
        self.stock.sell();
    }
}

struct Broker<'a> {
    order_list: Vec<Box<&'a dyn Order>>
}

impl <'a>Broker<'a> {
    fn new() -> Broker<'a> {
        Broker { order_list: vec![] }
    }
}

impl<'a> Broker<'a> {
    fn take_order(&mut self, order: Box<&'a dyn Order>) {
        self.order_list.push(order);
    }

    fn place_orders(&mut self) {
        {
            for item in self.order_list.iter() {
                item.execute();
            }
        }
        self.order_list.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command() {
        let stock = Stock::new(String::from("ABC"), 10);
        let buy_stock_order = BuyStock::new(&stock);
        let sell_stock_order = SellStock::new(&stock);
        let mut broker = Broker::new();
        broker.take_order(Box::new(&buy_stock_order));
        broker.take_order(Box::new(&sell_stock_order));
        broker.place_orders();
    }
}

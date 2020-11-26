use std::rc::Rc;

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

struct BuyStock {
    stock: Rc<Stock>
}

impl BuyStock {
    fn new(stock: Rc<Stock>) -> BuyStock {
        BuyStock { stock }
    }
}

impl Order for BuyStock {
    fn execute(&self) {
        self.stock.buy();
    }
}

struct SellStock {
    stock: Rc<Stock>
}

impl SellStock {
    fn new(stock: Rc<Stock>) -> SellStock {
        SellStock { stock }
    }
}

impl Order for SellStock {
    fn execute(&self) {
        self.stock.sell();
    }
}

struct Broker {
    order_list: Vec<Box<dyn Order>>
}

impl Broker {
    fn new() -> Broker {
        Broker { order_list: vec![] }
    }
}

impl Broker {
    fn take_order(&mut self, order: Box<dyn Order>) {
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
        let stock = Rc::new(Stock::new(String::from("ABC"), 10));
        let buy_stock_order = BuyStock::new(stock.clone());
        let sell_stock_order = SellStock::new(stock.clone());
        let mut broker = Broker::new();
        broker.take_order(Box::new(buy_stock_order));
        broker.take_order(Box::new(sell_stock_order));
        broker.place_orders();
    }
}

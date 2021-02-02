//!
//! 在策略模式（Strategy Pattern）中，一个类的行为或其算法可以在运行时更改。
//! 这种类型的设计模式属于行为型模式。
//!
//! 在策略模式中，我们创建表示各种策略的对象和一个行为随着策略对象改变而改变的 context 对象。
//! 策略对象改变 context 对象的执行算法。
//!

trait Strategy {
    fn do_operate(&self, num_1: i32, num_2: i32) -> i32;
}

struct OperationAdd;

struct OperationSubtract;

struct OperationMultiply;

impl OperationAdd {
    fn new() -> OperationAdd {
        OperationAdd
    }
}

impl Strategy for OperationAdd {
    fn do_operate(&self, num_1: i32, num_2: i32) -> i32 {
        num_1 + num_2
    }
}

impl OperationSubtract {
    fn new() -> OperationSubtract {
        OperationSubtract
    }
}

impl Strategy for OperationSubtract {
    fn do_operate(&self, num_1: i32, num_2: i32) -> i32 {
        num_1 - num_2
    }
}

impl OperationMultiply {
    fn new() -> OperationMultiply {
        OperationMultiply
    }
}

impl Strategy for OperationMultiply {
    fn do_operate(&self, num_1: i32, num_2: i32) -> i32 {
        num_1 * num_2
    }
}

struct Context {
    strategy: Box<dyn Strategy>
}

impl Context {
    fn new(strategy: Box<dyn Strategy>) -> Context {
        Context { strategy }
    }
}

impl Context {
    fn execute_strategy(&self, num_1: i32, num_2: i32) -> i32 {
        self.strategy.do_operate(num_1, num_2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy() {
        let context = Context::new(Box::new(OperationAdd::new()));
        println!("9 + 3 = {}", context.execute_strategy(9, 3));

        let context = Context::new(Box::new(OperationSubtract::new()));
        println!("9 - 3 = {}", context.execute_strategy(9, 3));

        let context = Context::new(Box::new(OperationMultiply::new()));
        println!("9 * 3 = {}", context.execute_strategy(9, 3));
    }
}

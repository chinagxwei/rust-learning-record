//!
//! 解释器模式（Interpreter Pattern）提供了评估语言的语法或表达式的方式，它属于行为型模式。
//! 这种模式实现了一个表达式接口，该接口解释一个特定的上下文。
//! 这种模式被用在 SQL 解析、符号处理引擎等。
//!

trait Expression {
    fn interpret(&self, context: &'static str) -> bool;
}

struct TerminalExpression {
    data: &'static str
}

impl TerminalExpression {
    fn new(data: &'static str) -> TerminalExpression {
        TerminalExpression { data }
    }
}

impl Expression for TerminalExpression {
    fn interpret(&self, context: &'static str) -> bool {
        context.contains(self.data)
    }
}

struct OrExpression {
    expr_1: Box<dyn Expression>,
    expr_2: Box<dyn Expression>,
}

impl OrExpression {
    fn new(expr_1: Box<dyn Expression>, expr_2: Box<dyn Expression>) -> OrExpression {
        OrExpression { expr_1, expr_2 }
    }
}

impl Expression for OrExpression {
    fn interpret(&self, context: &'static str) -> bool {
        self.expr_1.interpret(context) || self.expr_2.interpret(context)
    }
}

struct AndExpression {
    expr_1: Box<dyn Expression>,
    expr_2: Box<dyn Expression>,
}

impl AndExpression {
    fn new(expr_1: Box<dyn Expression>, expr_2: Box<dyn Expression>) -> AndExpression {
        AndExpression { expr_1, expr_2 }
    }
}

impl Expression for AndExpression {
    fn interpret(&self, context: &'static str) -> bool {
        self.expr_1.interpret(context) && self.expr_2.interpret(context)
    }
}

struct InterpreterPatternDemo;

impl InterpreterPatternDemo {
    fn get_male_expression() -> Box<dyn Expression> {
        let robert = TerminalExpression::new("Robert");
        let john = TerminalExpression::new("John");
        Box::new(OrExpression::new(Box::new(robert), Box::new(john)))
    }

    fn get_married_woman_expression() -> Box<dyn Expression> {
        let julie = TerminalExpression::new("Julie");
        let married = TerminalExpression::new("Married");
        Box::new(AndExpression::new(Box::new(julie), Box::new(married)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpreter() {
        let is_male = InterpreterPatternDemo::get_male_expression();
        let is_married_woman = InterpreterPatternDemo::get_married_woman_expression();

        println!("John is male? {}", is_male.interpret("John"));
        println!("Julie is a married women? {}", is_married_woman.interpret("Married Julie"))
    }
}

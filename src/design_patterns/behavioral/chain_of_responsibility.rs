use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialOrd, PartialEq)]
enum LoggerLevel {
    INFO = 1,
    DEBUG,
    ERROR,
}

impl LoggerLevel {
    fn as_str(&self) -> &'static str {
        match *self {
            LoggerLevel::INFO => "INFO",
            LoggerLevel::DEBUG => "DEBUG",
            LoggerLevel::ERROR => "ERROR"
        }
    }
}

impl Display for LoggerLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

trait AbstractLogger {
    fn get_level(&self) -> LoggerLevel;

    fn get_next_logger(&self) -> Option<&Box<dyn AbstractLogger>>;

    fn set_next_logger(&mut self, next_logger: Box<dyn AbstractLogger>);

    fn write(&self, message: &str);

    fn log_message(&self, level: LoggerLevel, message: &str) {
        if self.get_level() <= level {
            self.write(message);
        }
        let next_logger = self.get_next_logger();
        if next_logger.is_some() {
            next_logger
                .unwrap()
                .log_message(level, message)
        }
    }
}

struct ConsoleLogger {
    level: LoggerLevel,
    next_logger: Option<Box<dyn AbstractLogger>>,
}

impl ConsoleLogger {
    fn new(level: LoggerLevel) -> ConsoleLogger {
        ConsoleLogger { level, next_logger: None }
    }
}

impl AbstractLogger for ConsoleLogger {
    fn get_level(&self) -> LoggerLevel {
        self.level
    }

    fn get_next_logger(&self) -> Option<&Box<dyn AbstractLogger>> {
        self.next_logger.as_ref()
    }

    fn set_next_logger(&mut self, next_logger: Box<dyn AbstractLogger>) {
        self.next_logger = Some(next_logger);
    }

    fn write(&self, message: &str) {
        println!("Standard Console::Logger: {}", message)
    }
}

struct ErrorLogger {
    level: LoggerLevel,
    next_logger: Option<Box<dyn AbstractLogger>>,
}

impl ErrorLogger {
    fn new(level: LoggerLevel) -> ErrorLogger {
        ErrorLogger { level, next_logger: None }
    }
}

impl AbstractLogger for ErrorLogger {
    fn get_level(&self) -> LoggerLevel {
        self.level
    }

    fn get_next_logger(&self) -> Option<&Box<dyn AbstractLogger>> {
        self.next_logger.as_ref()
    }

    fn set_next_logger(&mut self, next_logger: Box<dyn AbstractLogger>) {
        self.next_logger = Some(next_logger);
    }

    fn write(&self, message: &str) {
        println!("Error Console::Logger: {}", message)
    }
}

struct FileLogger {
    level: LoggerLevel,
    next_logger: Option<Box<dyn AbstractLogger>>,
}

impl FileLogger  {
    fn new(level: LoggerLevel) -> FileLogger  {
        FileLogger  { level, next_logger: None }
    }
}

impl AbstractLogger for FileLogger  {
    fn get_level(&self) -> LoggerLevel {
        self.level
    }

    fn get_next_logger(&self) -> Option<&Box<dyn AbstractLogger>> {
        self.next_logger.as_ref()
    }

    fn set_next_logger(&mut self, next_logger: Box<dyn AbstractLogger>) {
        self.next_logger = Some(next_logger);
    }

    fn write(&self, message: &str) {
        println!("File::Logger: {}", message)
    }
}

fn get_chain_of_loggers() -> Box<dyn AbstractLogger> {
    let mut error = ErrorLogger::new(LoggerLevel::ERROR);
    let mut console = ConsoleLogger::new(LoggerLevel::INFO);
    let file = FileLogger::new(LoggerLevel::INFO);

    console.set_next_logger(Box::new(file));
    error.set_next_logger(Box::new(console));
    Box::new(error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let chain = get_chain_of_loggers();

        chain.log_message(LoggerLevel::INFO, "This is an information.");

        chain.log_message(LoggerLevel::DEBUG, "This is a debug level information.");

        chain.log_message(LoggerLevel::ERROR, "This is an error information.")
    }
}

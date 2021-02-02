//!
//! 桥接（Bridge）是用于把抽象化与实现化解耦，使得二者可以独立变化。
//! 这种类型的设计模式属于结构型模式，它通过提供抽象化和实现化之间的桥接结构，来实现二者的解耦。
//!
//! 这种模式涉及到一个作为桥接的接口，使得实体类的功能独立于接口实现类。
//! 这两种类型的类可被结构化改变而互不影响。
//!
//! 我们通过下面的实例来演示桥接模式（Bridge Pattern）的用法。
//! 其中，可以使用相同的抽象类方法但是不同的桥接实现类，来画出不同颜色的圆。
//!

trait DrawAPI {
    fn draw_circle(&self, radius: i32, x: i32, y: i32);
}

struct RedCircle;

impl RedCircle {
    fn new() -> RedCircle {
        RedCircle
    }
}

impl DrawAPI for RedCircle {
    fn draw_circle(&self, radius: i32, x: i32, y: i32) {
        println!("Drawing Circle[ color: red, radius: {},x: {},y: {}]", radius, x, y);
    }
}

struct GreenCircle;

impl GreenCircle {
    fn new() -> GreenCircle {
        GreenCircle
    }
}

impl DrawAPI for GreenCircle {
    fn draw_circle(&self, radius: i32, x: i32, y: i32) {
        println!("Drawing Circle[ color: green, radius: {},x: {},y: {}]", radius, x, y);
    }
}

trait Shape {
    fn get_draw_api(&self) -> Box<&dyn DrawAPI>;
    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;
    fn get_radius(&self) -> i32;
    fn draw(&self) {
        self.get_draw_api()
            .draw_circle(
                self.get_radius(),
                self.get_x(),
                self.get_y(),
            )
    }
}

struct Circle {
    radius: i32,
    x: i32,
    y: i32,
    draw_api: Box<dyn DrawAPI>,
}

impl Circle {
    fn new(radius: i32, x: i32, y: i32, draw_api: Box<dyn DrawAPI>) -> Circle {
        Circle {
            radius,
            x,
            y,
            draw_api,
        }
    }
}

impl Shape for Circle {
    fn get_draw_api(&self) -> Box<&dyn DrawAPI> {
        Box::new(self.draw_api.as_ref())
    }

    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }

    fn get_radius(&self) -> i32 {
        self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge() {
        let red_circle = Circle::new(100, 100, 10, Box::new(RedCircle::new()));
        let green_circle = Circle::new(100, 100, 10, Box::new(GreenCircle::new()));
        red_circle.draw();
        green_circle.draw();
    }
}



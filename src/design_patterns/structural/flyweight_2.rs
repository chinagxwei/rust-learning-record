use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct Circle {
    color: &'static str,
    x: i32,
    y: i32,
    radius: i32,
}

impl Circle {
    fn new(color: &'static str) -> Circle {
        Circle {
            color,
            x: 0,
            y: 0,
            radius: 0,
        }
    }
}

impl Circle {
    fn set_x(&mut self, x: i32) {
        self.x = x;
    }
    fn set_y(&mut self, y: i32) {
        self.y = y;
    }
    fn set_radius(&mut self, radius: i32) {
        self.radius = radius;
    }
}

impl Circle {
    fn draw(&self) {
        println!("Circle: Draw() [Color : {}, x : {}, y : {}, radius : {}]", self.color, self.x, self.y, self.radius);
    }
}

struct ShapeFactory {
    circle_map: RefCell<HashMap<&'static str, Rc<RefCell<Circle>>>>
}

impl ShapeFactory {
    fn new() -> ShapeFactory {
        ShapeFactory { circle_map: RefCell::new(Default::default()) }
    }
}

impl ShapeFactory {
    fn get_circle(&self, color: &'static str) -> Rc<RefCell<Circle>> {
        self.circle_map
            .borrow_mut()
            .entry(color)
            .or_insert_with(|| {
                println!("Creating circle of color : {}", color);
                Rc::new(RefCell::new(Circle::new(color)))
            })
            .clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flyweight() {
        let colors = ["Red", "Green", "Blue", "White", "Black"];

        let get_random_color = |i: usize| -> &'static str { colors[(i % 5)] };

        let get_random_x = |i: usize| -> i32 { (i + 1) as i32 };

        let get_random_y = |i: usize| -> i32 { (i + 2) as i32 };

        let sf = ShapeFactory::new();

        for index in 0..20 {
            let c = sf.get_circle(get_random_color(index));
            c.borrow_mut().set_radius(100);

            c.borrow_mut().set_x(get_random_x(index));

            c.borrow_mut().set_y(get_random_y(index));

            c.borrow().draw();
        }
    }
}

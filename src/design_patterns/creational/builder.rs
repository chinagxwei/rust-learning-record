trait Packing {
    fn pack(&self) -> &'static str;
}

trait Item {
    fn name(&self) -> &'static str;
    fn packing(&self) -> Box<dyn Packing>;
    fn price(&self) -> i64;
}

struct Wrapper;

impl Wrapper {
    fn new() -> Wrapper {
        Wrapper
    }
}

impl Packing for Wrapper {
    fn pack(&self) -> &'static str {
        "Wrapper"
    }
}

struct Bottle;

impl Bottle {
    fn new() -> Bottle {
        Bottle
    }
}

impl Packing for Bottle {
    fn pack(&self) -> &'static str {
        "Bottle"
    }
}

struct VegBurger;

impl VegBurger {
    fn new() -> VegBurger {
        VegBurger
    }
}

impl Item for VegBurger {
    fn name(&self) -> &'static str {
        "Veg Burger"
    }

    fn packing(&self) -> Box<dyn Packing> {
        Box::new(Wrapper::new())
    }

    fn price(&self) -> i64 {
        25_i64
    }
}

struct ChickenBurger;

impl ChickenBurger {
    fn new() -> ChickenBurger {
        ChickenBurger
    }
}

impl Item for ChickenBurger {
    fn name(&self) -> &'static str {
        "Chicken Burger "
    }

    fn packing(&self) -> Box<dyn Packing> {
        Box::new(Wrapper::new())
    }

    fn price(&self) -> i64 {
        50_i64
    }
}

struct Coke;

impl Coke {
    fn new() -> Coke {
        Coke
    }
}

impl Item for Coke {
    fn name(&self) -> &'static str {
        "Coke"
    }

    fn packing(&self) -> Box<dyn Packing> {
        Box::new(Bottle::new())
    }

    fn price(&self) -> i64 {
        30_i64
    }
}

struct Pepsi;

impl Pepsi {
    fn new() -> Pepsi {
        Pepsi
    }
}

impl Item for Pepsi {
    fn name(&self) -> &'static str {
        "Pepsi"
    }

    fn packing(&self) -> Box<dyn Packing> {
        Box::new(Bottle::new())
    }

    fn price(&self) -> i64 {
        35_i64
    }
}

struct Meal {
    items: Vec<Box<dyn Item>>
}

impl Meal {
    fn new() -> Meal {
        Meal { items: vec![] }
    }
}

impl Meal {
    fn add_item(&mut self, item: Box<dyn Item>) {
        self.items.push(item)
    }

    fn get_cost(&self) -> i64 {
        let mut cost = 0_i64;
        for i in self.items.iter() {
            cost += i.price()
        }
        cost
    }

    fn show_item(&self) {
        for i in self.items.iter() {
            println!(
                "Item : {}, Packing : {}, Price : {}",
                i.name(),
                i.packing().pack(),
                i.price()
            )
        }
    }
}

struct MealBuilder;

impl MealBuilder {
    fn prepare_veg_meal() -> Meal {
        let mut meal = Meal::new();
        meal.add_item(Box::new(VegBurger::new()));
        meal.add_item(Box::new(Coke::new()));
        meal
    }

    fn prepare_non_veg_meal() -> Meal {
        let mut meal = Meal::new();
        meal.add_item(Box::new(ChickenBurger::new()));
        meal.add_item(Box::new(Pepsi::new()));
        meal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let veg_meal = MealBuilder::prepare_veg_meal();
        println!("Veg Meal");
        veg_meal.show_item();
        println!("Total Cost: {}", veg_meal.get_cost());

        let non_veg_meal = MealBuilder::prepare_non_veg_meal();
        println!("\n\nNon-Veg Meal");
        non_veg_meal.show_item();
        println!("Total Cost: {}", non_veg_meal.get_cost())
    }
}




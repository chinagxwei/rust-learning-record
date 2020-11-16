use std::rc::{Rc, Weak};
use std::fmt::{Display, Formatter};
use std::fmt;
use std::cell::RefCell;
use std::borrow::Borrow;

#[derive(Debug, Copy, Clone)]
enum CarType {
    CityCar,
    SportCar,
    SUV,
}

impl Display for CarType {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CarType::CityCar => write!(fmt, "{}", "CityCar"),
            CarType::SportCar => write!(fmt, "{}", "SportCar"),
            CarType::SUV => write!(fmt, "{}", "SUV")
        }
    }
}


#[derive(Debug, Copy, Clone)]
enum Transmission {
    SingleSpeed,
    Manual,
    Automatic,
    SemiAutomatic,
}

impl Display for Transmission {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Transmission::SingleSpeed => write!(fmt, "{}", "SingleSpeed"),
            Transmission::Manual => write!(fmt, "{}", "Manual"),
            Transmission::Automatic => write!(fmt, "{}", "Automatic"),
            Transmission::SemiAutomatic => write!(fmt, "{}", "SemiAutomatic")
        }
    }
}

trait Builder<T> {
    fn set_car_type(&mut self, car_type: CarType);
    fn set_seats(&mut self, seats: i32);
    fn set_engine(&mut self, engine: Engine);
    fn set_transmission(&mut self, transmission: Transmission);
    fn set_trip_computer(&mut self, trip_computer: TripComputer);
    fn set_gps_navigator(&mut self, gps_navigator: GPSNavigator);
    fn build(&mut self) -> T;
}

#[derive(Debug, Clone)]
struct GPSNavigator {
    route: String
}

impl GPSNavigator {
    fn new() -> GPSNavigator {
        GPSNavigator { route: "221b, Baker Street, London  to Scotland Yard, 8-10 Broadway, London".to_string() }
    }

    fn new_by_manual(manual_route: String) -> GPSNavigator {
        GPSNavigator { route: manual_route }
    }

    fn get_route(&self) -> String {
        self.route.to_owned()
    }
}

struct TripComputer {
    car: Option<RefCell<Weak<Car>>>
}

impl TripComputer {
    fn set_car(&mut self, car: Option<RefCell<Weak<Car>>>) {
        self.car = car
    }

    fn show_fuel_level(&self) {
        let car = self.car.as_ref().unwrap().borrow().upgrade();
        if car.is_some() {
            println!("Fuel level: {}", car.as_ref().unwrap().get_fuel());
        }
    }

    fn show_status(&self) {
        let car = self.car.as_ref().unwrap().borrow().upgrade();
        if car.is_some() && car.as_ref().unwrap().get_engine().is_started() {
            println!("Car is started")
        } else {
            println!("Car isn't started")
        }
    }
}

struct Car {
    car_type: CarType,
    seats: i32,
    engine: Engine,
    transmission: Transmission,
    trip_computer: TripComputer,
    gps_navigator: GPSNavigator,
    fuel: f64,
}

impl Car {
    fn new(
        car_type: CarType,
        seats: i32,
        engine: Engine,
        transmission: Transmission,
        trip_computer: TripComputer,
        gps_navigator: GPSNavigator,
    ) -> Car {
        Car {
            car_type,
            seats,
            engine,
            transmission,
            trip_computer,
            gps_navigator,
            fuel: 0.0,
        }
    }
}

impl Car {
    fn get_car_type(&self) -> CarType {
        return self.car_type;
    }

    fn get_engine(&self) -> &Engine {
        &self.engine
    }

    fn get_seats(&self) -> i32 {
        self.seats
    }

    fn get_transmission(&self) -> &Transmission {
        &self.transmission
    }

    fn get_trip_computer(&self) -> &TripComputer {
        &self.trip_computer
    }

    fn get_gps_navigator(&self) -> &GPSNavigator {
        &self.gps_navigator
    }

    fn set_fuel(&mut self, fuel: f64) {
        self.fuel = fuel;
    }

    fn get_fuel(&self) -> f64 {
        self.fuel
    }
}

struct Manual {
    car_type: CarType,
    seats: i32,
    engine: Engine,
    transmission: Transmission,
    trip_computer: Option<TripComputer>,
    gps_navigator: Option<GPSNavigator>,
}

impl Manual {
    fn new(
        car_type: CarType,
        seats: i32,
        engine: Engine,
        transmission: Transmission,
        trip_computer: Option<TripComputer>,
        gps_navigator: Option<GPSNavigator>,
    ) -> Manual {
        Manual {
            car_type,
            seats,
            engine,
            transmission,
            trip_computer,
            gps_navigator,
        }
    }
}

impl Manual {
    fn print(&self) -> String {
        let mut info = format!(
            "Type of car: {}\nCount of seats: {}\nEngine: volume - {}; mileage - {}\nTransmission: {}\n",
            self.car_type,
            self.seats,
            self.engine.volume,
            self.engine.mileage,
            self.transmission
        );

        if self.trip_computer.is_some() {
            info = format!("{}Trip Computer: Functional\n", info);
        } else {
            info = format!("{}Trip Computer: N/A\n", info);
        }
        if self.gps_navigator.is_some() {
            info = format!("{}GPS Navigator: Functional\n", info);
        } else {
            info = format!("{}GPS Navigator: N/A\n", info);
        }
        info
    }
}

#[derive(Debug, Clone)]
struct Engine {
    volume: f64,
    mileage: f64,
    started: bool,
}

impl Engine {
    fn new(volume: f64, mileage: f64) -> Engine {
        Engine {
            volume,
            mileage,
            started: false,
        }
    }
}

impl Engine {
    fn on(&mut self) {
        self.started = true;
    }

    fn off(&mut self) {
        self.started = false;
    }

    fn is_started(&self) -> bool {
        self.started
    }

    fn go(&mut self, mileage: f64) {
        if self.started {
            self.mileage = mileage;
        } else {
            println!("Cannot go(), you must start engine first!")
        }
    }

    fn get_volume(&self) -> f64 {
        self.volume
    }

    fn get_mileage(&self) -> f64 {
        self.mileage
    }
}

struct CarBuilder {
    car_type: Option<CarType>,
    seats: Option<i32>,
    engine: Option<Engine>,
    transmission: Option<Transmission>,
    trip_computer: Option<TripComputer>,
    gps_navigator: Option<GPSNavigator>,
}

impl CarBuilder {
    fn new() -> CarBuilder {
        CarBuilder {
            car_type: None,
            seats: None,
            engine: None,
            transmission: None,
            trip_computer: None,
            gps_navigator: None,
        }
    }
}

impl Builder<Car> for CarBuilder {
    fn set_car_type(&mut self, car_type: CarType) {
        self.car_type = Option::from(car_type);
    }

    fn set_seats(&mut self, seats: i32) {
        self.seats = Option::from(seats);
    }

    fn set_engine(&mut self, engine: Engine) {
        self.engine = Option::from(engine);
    }

    fn set_transmission(&mut self, transmission: Transmission) {
        self.transmission = Option::from(transmission);
    }

    fn set_trip_computer(&mut self, trip_computer: TripComputer) {
        self.trip_computer = Option::from(trip_computer);
    }

    fn set_gps_navigator(&mut self, gps_navigator: GPSNavigator) {
        self.gps_navigator = Option::from(gps_navigator);
    }

    fn build(&mut self) -> Car {
        Car::new(
            self.car_type.take().unwrap(),
            self.seats.take().unwrap(),
            self.engine.take().unwrap(),
            self.transmission.take().unwrap(),
            self.trip_computer.take().unwrap(),
            self.gps_navigator.take().unwrap(),
        )
    }
}

struct CarManualBuilder {
    car_type: Option<CarType>,
    seats: Option<i32>,
    engine: Option<Engine>,
    transmission: Option<Transmission>,
    trip_computer: Option<TripComputer>,
    gps_navigator: Option<GPSNavigator>,
}

impl CarManualBuilder {
    fn new() -> CarManualBuilder {
        CarManualBuilder {
            car_type: None,
            seats: None,
            engine: None,
            transmission: None,
            trip_computer: None,
            gps_navigator: None,
        }
    }
}

impl Builder<Manual> for CarManualBuilder {
    fn set_car_type(&mut self, car_type: CarType) {
        self.car_type = Option::from(car_type);
    }

    fn set_seats(&mut self, seats: i32) {
        self.seats = Option::from(seats);
    }

    fn set_engine(&mut self, engine: Engine) {
        self.engine = Option::from(engine);
    }

    fn set_transmission(&mut self, transmission: Transmission) {
        self.transmission = Option::from(transmission);
    }

    fn set_trip_computer(&mut self, trip_computer: TripComputer) {
        self.trip_computer = Option::from(trip_computer);
    }

    fn set_gps_navigator(&mut self, gps_navigator: GPSNavigator) {
        self.gps_navigator = Option::from(gps_navigator);
    }

    fn build(&mut self) -> Manual {
        Manual::new(
            self.car_type.take().unwrap(),
            self.seats.take().unwrap(),
            self.engine.take().unwrap(),
            self.transmission.take().unwrap(),
            self.trip_computer.take(),
            self.gps_navigator.take(),
        )
    }
}

struct Director;

impl Director {
    fn construct_sport_car<T>(mut builder: Box<dyn Builder<T>>) -> Box<dyn Builder<T>> {
        builder.set_car_type(CarType::SportCar);
        builder.set_seats(2);
        builder.set_engine(Engine::new(3.0, 0.0));
        builder.set_transmission(Transmission::SemiAutomatic);
        builder.set_trip_computer(TripComputer { car: None });
        builder.set_gps_navigator(GPSNavigator::new());
        builder
    }

    fn construct_city_car<T>(mut builder: Box<dyn Builder<T>>) -> Box<dyn Builder<T>> {
        builder.set_car_type(CarType::CityCar);
        builder.set_seats(2);
        builder.set_engine(Engine::new(1.2, 0.0));
        builder.set_transmission(Transmission::Automatic);
        builder.set_trip_computer(TripComputer { car: None });
        builder.set_gps_navigator(GPSNavigator::new());
        builder
    }

    fn construct_suv<T>(mut builder: Box<dyn Builder<T>>) -> Box<dyn Builder<T>> {
        builder.set_car_type(CarType::SUV);
        builder.set_seats(4);
        builder.set_engine(Engine::new(2.5, 0.0));
        builder.set_transmission(Transmission::Manual);
        builder.set_trip_computer(TripComputer { car: None });
        builder.set_gps_navigator(GPSNavigator::new());
        builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let car_build = CarBuilder::new();

        let car = Director::construct_sport_car::<Car>(Box::new(car_build)).build();

        println!("Car built: \n{}", car.get_car_type());

        let car_manual_builder = CarManualBuilder::new();

        let manual = Director::construct_sport_car::<Manual>(Box::new(car_manual_builder)).build();

        println!("\nCar manual built:\n{}", manual.print())
    }
}

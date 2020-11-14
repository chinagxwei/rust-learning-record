enum CarType {
    CityCar,
    SportCar,
    SUV,
}

enum Transmission {
    SingleSpeed,
    Manual,
    Automatic,
    SemiAutomatic,
}

trait Builder {
    fn set_car_type(&mut self, car_type: CarType);
    fn set_seats(&mut self, seats: i32);
    fn set_engine(&mut self, engine: Engine);
    fn set_transmission(&mut self, transmission: Transmission);
    fn set_trip_computer(&mut self, trip_computer: TripComputer);
    fn set_gps_navigator(&mut self, gps_navigator: GPSNavigator);
}

struct Engine {
    volume: f64,
    mileage: f64,
    started: bool,
}

struct TripComputer {
    car: Car
}

struct GPSNavigator {
    route: String
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

struct Manual {
    car_type: CarType,
    seats: i32,
    engine: Engine,
    transmission: Transmission,
    trip_computer: TripComputer,
    gps_navigator: GPSNavigator,
}

struct CarBuilder {
    car_type: CarType,
    seats: i32,
    engine: Engine,
    transmission: Transmission,
    trip_computer: TripComputer,
    gps_navigator: GPSNavigator,
}

struct CarManualBuilder {
    car_type: CarType,
    seats: i32,
    engine: Engine,
    transmission: Transmission,
    trip_computer: TripComputer,
    gps_navigator: GPSNavigator,
}

struct Director;

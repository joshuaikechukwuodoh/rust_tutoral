// 1. Struct: Blueprint for creating objects (like designing a car)
struct Car {
    engine: String,
    wheels: i32,
}

// 2. Impl: Adding actions or behaviors to the Car (like starting the car)
impl Car {
    fn start(&self) {
        println!("The car with engine {} is starting.", self.engine);
    }

    fn stop(&self) {
        println!("The car is stopping.");
    }
}

// 3. Trait: A set of rules (like saying all Vehicles must start and stop)
trait Vehicle {
    fn start(&self);  // Vehicles must have a start method
    fn stop(&self);   // Vehicles must have a stop method
}

// Implementing the Vehicle trait for the Car
impl Vehicle for Car {
    fn start(&self) {
        println!("The car with engine {} is starting (from trait).", self.engine);
    }

    fn stop(&self) {
        println!("The car is stopping (from trait).");
    }
}

fn main() {
    // Creating a car using the Car struct
    let my_car = Car {
        engine: String::from("V8"),
        wheels: 4,
    };

    // Using the methods directly from impl
    my_car.start(); // Output: The car with engine V8 is starting.
    my_car.stop();  // Output: The car is stopping.

    // Using the methods from the Vehicle trait
    my_car.start(); // Output: The car with engine V8 is starting (from trait).
    my_car.stop();  // Output: The car is stopping (from trait).
}

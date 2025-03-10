/// Module for demonstrating basic Rust features
pub mod variables {
    /// Demonstrates variable declaration and mutation
    pub fn variable_examples() {
        // Immutable variable (default)
        let x = 5;
        println!("The value of x is: {}", x);
        
        // Mutable variable
        let mut y = 5;
        println!("The value of y is: {}", y);
        y = 6;
        println!("The value of y has changed to: {}", y);
        
        // Constants
        const MAX_POINTS: u32 = 100_000;
        println!("The maximum points is: {}", MAX_POINTS);
        
        // Shadowing
        let z = 5;
        let z = z + 1;  // Creates a new variable that shadows the previous one
        println!("The value of z is: {}", z);
    }
}

/// Module for demonstrating Rust's data types
pub mod data_types {
    /// Shows examples of primitive data types
    pub fn type_examples() {
        // Integer types
        let signed: i32 = -42;
        let unsigned: u32 = 42;
        
        // Floating point
        let float: f64 = 3.14159;
        
        // Boolean
        let is_active: bool = true;
        
        // Character
        let emoji = '😊';
        
        println!("Signed: {}, Unsigned: {}", signed, unsigned);
        println!("Float: {}", float);
        println!("Boolean: {}", is_active);
        println!("Character: {}", emoji);
        
        // Compound types
        // Tuple
        let tuple: (i32, f64, char) = (500, 6.4, 'z');
        println!("Tuple values: {}, {}, {}", tuple.0, tuple.1, tuple.2);
        
        // Array
        let array = [1, 2, 3, 4, 5];
        println!("Array first element: {}", array[0]);
    }
}

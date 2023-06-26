use std::fmt::Display;
use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

impl Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() { 
    let value = Complex { real: 3.3, imag: 7.2 };

    println!("Display: {}", value);
    println!("Debug: {:?}", value);
}

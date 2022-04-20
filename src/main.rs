mod person;

use crate::person::routes;

fn main() {
    println!("Hello {}", routes::get());
}

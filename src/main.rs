

// App Layers
mod domain;
mod controllers;
mod services;
mod respositories;

// Imports
use domain::user;

fn main() {
    println!("Tests");

    let d = user::User {
        _id: String::from("12345"),
        name: String::from("ertert"),
        phone: 13445,
        active: true,
    };

    print!("User is: {:?}", d);
}
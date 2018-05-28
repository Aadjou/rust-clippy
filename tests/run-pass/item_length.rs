#![deny(item_length)]

struct Pair(i32, i32);

struct Parents {
	mother: Animal,
	father: Animal,
}

struct Animal {
    name: String,
    age: u8,
    breed: Breed,
}

enum Breed {
	Sphynx,
	BritishShorthair,
}

fn main() {
 	let name = "Peter".to_string();
    let age = 27;
    let breed = Breed::Sphynx;
    let pet = Animal { name, age, breed };

    println!("I have a pet, with the name { } .", pet.name );
}
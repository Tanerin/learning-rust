pub fn run() {
    let age = 20;
    //IF/Else
    if age >= 21 {
        println!("Barhender: what woul you like to drink!");

    }else{
        println!("Barhender: Sorry, you have to leave");
    }
    //shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
    
}
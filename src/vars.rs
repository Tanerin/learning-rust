//Variables hold primitive data or references to data
//Variables are inmutable by default
//Rust is a block-scoped language
pub fn run() {
    //let inmutable
    let name = "Ulises";
    //let mutatble
    let mut age= 20;
    println!("My name is:{} and I am {}",name, age);
    age = 21;
    println!("My name is:{} and I am {}",name, age);

    //Define constants
    const ID: i32 = 001;
    println!("ID: {}",ID);

    //Assing multiple Variables
    let (my_name, my_age) = ("Ulises", 20);
    println!("{}, {}",my_name,my_age);
}
pub fn run() {
    let person: (&str, &str, i8) = ("Ulises", "Mexico", 21);
    println!("{} is from {} an is {}", person.0, person.1, person.2);
}
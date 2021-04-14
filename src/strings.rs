pub fn run() {
    let hello ="Hello";
    let mut hello_str = String::from("Hello ");
    // Get length    
    println!("Length: {}",hello.len());
    hello_str.push('W');
    hello_str.push_str("orld!");
    println!("Capacity: {}", hello_str.capacity());
    for word in hello_str.split_whitespace() {
        println!("{}",word);
    }
    println!("{}",hello_str);

}
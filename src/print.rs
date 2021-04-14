pub fn run() {
    //print to console
    println!("Hello from the print.rs");
    // Basic Formating
    println!("Number: {}",1);
    println!("Numbers: {} {}",1,2);
    println!("{0} is from {1} and {0} likes to {2}","Ulises","Mexico","code");
    //Named arguments
    println!("{name} likes to play {game}",name="Ulises", game="Minecraft");
    //Placeholder traits
    println!("Bianry :{:b} Hex: {:x} Ocatal: {:o}",10,10,10);

    //Placeholder for debug trait  
    println!("{:?}",(12,true,"hello"));

    //Basic math 
    println!("10 + 10 = {}",10+10);
}
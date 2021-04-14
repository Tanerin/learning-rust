//Integers u8,i8,u16,i16,u32,i32,u64,i64,u128,i128
//Floats f32,f64
//Boolean (bool)
//Characters (char)
//Tuples
//Arrays 

pub fn run() {
    //Integer "i32"
    let x = 1;
    //Float is "f64"    
    let y =2.5;
    //add explicit type
    let z: i64 = 45454545454545;
    //Find max size
    println!("Max i32: {}",std::i32::MAX);
    println!("Max i64: {}",std::i64::MAX);
    //Boolean
    let is_active: bool = true;
    let is_greater = 10 <= 5;
    let a1 = 'a';
    println!("{:?}",(x,y,z, is_active, is_greater,a1));
}
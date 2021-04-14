pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5,6,7,8,9];
    numbers[2]=20;
    //Add vector values to vector
    numbers.push(5);
    numbers.push(6);
    numbers.push(8);
    print!("{:?}", numbers);
    //Drop the last element of the vector    
    numbers.pop();
    println!("{:?}", numbers);
    //Loop trough vectors
    for x in numbers.iter() {
        println!("Number: {}", x);
    }
    for x in numbers.iter_mut() {
        *x *=2;
    }
    println!("Numbers Vec:{:?}", numbers)
}
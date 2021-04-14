pub fn run() {
    let array1= [1,2,3];
    let array2= array1;
    
    println!("{:?}", (array1,array2));

    let vec1 = vec![1,2,3];
    let vec2 = &vec1;
    println!("{:?}",(&vec1,vec2));
}
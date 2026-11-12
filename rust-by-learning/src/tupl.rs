pub fn reverse(pair: (i32,bool)) -> (bool, i32){

    let (integer, boolean_value) = pair;

    println!("{} {} ", boolean_value, integer);
    

    let long_tuple = (1u8, 4u16,5u32,6u64,-8i8, -8i16,-16i32,-24i64);

    println!("The first value is {}", long_tuple.0);
    println!("The second value is {}", long_tuple.1);

    #[derive(Debug)]
    struct Matrix(f32,f32,f32,f32);


    let tuple_of_tuples = ((20u8,50u32), (40u8,60u16), 10u8);

    println!("tuple of tuples {:?}", tuple_of_tuples);

    println!("one element tuple {:?}", (50u8,));
    println!("one element tuple without the comma {:?}", (50u8));

    let items = (1i8, "hello", 4.5f32, true);

    let (number , greeting, data, is_analytics) = items;

    println!("{:?} , {:?} , {:?} , {:?}", number, greeting, data, is_analytics);

    let matrix = Matrix(1.2,2.1,4.6,4.8);

    println!("This is also a tuple {:?}", matrix);

    return (boolean_value, integer);
}

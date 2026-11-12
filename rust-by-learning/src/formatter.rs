pub fn format(){
    println!("Hello world");
    println!("{0}, this is {1}, {1} this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}", subject="The Quick Brown fox", verb="jumps over", object="the lazy dog");


    println!("Base 10:  {}", 3000);
    println!("Base 2:  {:04b}",  0x8f32i32);
    println!("Octal: {:o}",  0b0011i32);
    println!("Hex:  {:x}", 300i32);
    println!("{number:>5}", number="1");
    println!("{number:<5}", number="5");
    println!("{number:0<5}", number="6");
    println!("{number:0<width$}", number="5", width=4);

    
    #[derive(Debug)]
    struct Structure(i32);


    println!("this is {:?}", Structure(3));
    //
    
    let number: f32  = 1.5;
    let width: usize = 5;

    println!("{number:>width$}");
}


use std::vec;



fn main() {
    
    use itertools::max;
    use itertools::min;
    use rand::Rng;

    let mut vec: Vec<i32>  = (1..100).collect();

    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..len(vec));

    match i {

        Some(1) => println!("One"),
        Some(2|3|5|7|11|13|19) => println!("Prime"),
        Some(2) => print!("2 digits"),
        
        _ => println!("Another")
    } 

}

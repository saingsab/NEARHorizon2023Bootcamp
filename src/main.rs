#[warn(unused_variables)]
fn main() {
    // Normall type
   let my_num = 5;
   let my_bool: bool = true;
   let my_char: char = 'ច';

   //This type of string store in the heap
   let my_string: String = "This is a String".to_string(); 

   // Unalocated somwhere in the storage.
   let my_str: &str = "This a &str";
   let my_str2: &str = &my_string;

   let my_f64_array: [f64; 5] = [5.0, 13.0, 2.5, 44.789, -27.1];
   let my_bool_array: [bool; 3] = [true, true, false];

// access to array in specific index
   let num: &[f64] = &my_f64_array[2..];

   let my_tuple = (5i32, true, [1, 2, 3], (true, "Hello"));

   let mut my_vec: Vec<i32> = vec![1, 2, 3];

    //    pushing item in to vec
    Vec::push(&mut my_vec, 42);
    println!("{my_vec:?}");

    struct Book {
        title: Box<str>,
        author: String,
        published_year: u32,
    }

    // control flow
    
    let mut x = 2;
    if 2 == x {
        println!("match works");
    } else {
        println!("else statment");
    }

    // for i int 0.. = 10 {
    //     println!("{i}");
    // }
    let my_no = 2000u32;
    let my_u8 = u8::try_from(my_no);

    match my_u8 {
        Ok(converted_value) => {
            println!("{converted_value}");
        }
        Err(error_value) => {
            println!("{error_value:?}");
        }
    }

    println!("{}", fib(5));
    // similar to empty tuple or void type of c
    fn print_greeting(name: &str) -> () {
        println!("Hello, {name}!");
    }

    let x = print_greeting("Encode club");
    println!("{x:?}");

    fn wrap<T>(val: T) ->Option<T> {
        Some(val)
    }

    let x: Option<i32> = wrap(1);
    let x: Option<&str> = wrap("Hello");

}

fn fib(n: usize) -> u64 {
    if n == 0 || n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    };
}

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

}

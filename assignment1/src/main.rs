fn main() {
/*
 1. ASSIGNMENT1
   *print out your name
   *print out your Mat-no
   *print your favourite emoji
   *divide two numbers and let the result be a fraction
*/

let my_name:&str="BASSEY FAVOUR";
println!("My name is {}",my_name);
    

let mat_no:&str="ENG2102757";//declearing a string &str.
println!("My Mat-no is:{}",mat_no);//printing out a string

let unamused_emoji="\u{1F612}";//declearing the unicode for an emoji.
println!("My Favourite emoji is-{}",unamused_emoji);//printing out the emoji
    
let first_no:f32=51.0;//declearing of a decimal value using float f32.
let second_no:f32=2.0;//declearing of a decimal value using float f32.
println!("First-no:{}",first_no);//prompting values
println!("second-no:{}",second_no);
println!("The result for dividing the first and second no is: {}",first_no/second_no);//printing out the results after dividing values.
   //DON'T START A VARIABLE NAME WITH CAPITAL LETTER 
}

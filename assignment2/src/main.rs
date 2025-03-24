fn main() {
/* 
 1. ASSIGNMENT1
 *print out your name
 *print out your Mat-no
 *print your favourite emoji
 *divide two numbers and let the result be a fraction
*/
    let my_name="BASSEY FAVOUR";
    name(my_name);//callimg the name function.

    let my_matno="ENG2102757";
    mat_no(my_matno);//calling the mat-no function.

    let unamused_emoji= "\u{1F612}";
    emoji(unamused_emoji);//calling the emoji function.

    let first_no:f32=42.8;
    let second_no:f32=3.0;
    println!("First-no:{}",first_no);//p
    println!("Second-no:{}",second_no);
   let result=divide_numbers(first_no,second_no);
   //calling the division function.
    println!("The result for the divided numbers is: {}",result);
//printing out the result for the division.
}
fn name(name:&str){
    println!("My name is {}",name);
    //function for printing out my name.
}

fn mat_no(mat_no:&str){
    println!("My mat-no is: {}",mat_no);
    //function for printing out my mat-no.
}

fn emoji(emoji:&str){
    println!("My favourite emoji is-{}",emoji);
// function for printing my favourite emoji.
}
fn divide_numbers(num1:f32,num2:f32)->f32{
    num1/num2
    //function for dividing 2 numbers.
}


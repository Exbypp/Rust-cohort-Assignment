fn main() { //learning compound types.
//arrays.
//tupples.
    //println!("Hello, world!\n my name is emerald.");
let scores_1:[i32;3]=[85,90,21];   
/*let sum:i32=scores_1.iter().sum();
println!("The sum of angel's courses is {}",sum);
let scores_1:[i32;3]=[85,90,21];
let length:i32=scores_1.len().try_into().unwrap();
println!("The length is {}",length);
let average:i32=sum/length;
println!("The  average scores angel got is {}",average);
*/
/*let student_1:(&str,u32,bool)=("Angel",21,true);

let scores_2:[i32;3]=[84,35,93];   

let student_2:(&str,u32,bool)=("emmy",34,false);
println!("This is the information for {} in CPE591 is {}, CPE 491 is {} and CPE392 is {}",student_1.0,scores_1[0],scores_1[1],scores_1[2]);
*/
let final_score:i32=summation(&scores_1);
println!("The sum is {}",final_score);

 }

fn summation(scores_1:&[i32]){
    scores_1.iter().sum
}


/*let sum:i32=add();
println!("The sum of the scores angel got is {}",sum);
let average:f32=divide();
println!("The average of the scores angel got is {}",average);
}
fn add(c1:i32,c2:i32,c3:i32){ 
    c1+c2+c3

}
fn divide(c1:f32,c2:f32,c3:f32){
    (c1+c2+c3)/3
}*/
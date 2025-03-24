/*
        ASSIGNEMNT-2
  *Using method to find the sum of scores
  *using method to solve for the average of the scores.
  *find the highest scoring student.
  *indicate the students that passed and failed.
*/

fn main() {
   let scores:[i32;10]=[56,45,87,90,34,76,97,65,82,19];
   //array containing 10 values.
   let f_score:i32=scores.iter().sum();// method for summation-total scores.
   println!("The sum of the total scores is {}",f_score);
   let average:i32=scores.len().try_into().unwrap();
   //The method to dolve for average of values.
   println!("THe average scores of the students is {}",average);
   let max_score=scores.iter().max().unwrap();
   //Method for finding the maximum values amongst a set of values in an array.
   println!("The score of the highest scoring student is {}",max_score)
    ;
    println!("The scores of failed students are :");
    for values in scores.iter() // The method for finding /sorting the values in an array.
    {
            if *values<=50{
                println!("{}",values);//if condition to print out a list of values.
            }

    }
    println!("The scores of students that passed are:");
    for value in scores.iter(){// method for looping through a setbof values
            if *value>50{
                println!("{}",value);
            }
    }
}


   

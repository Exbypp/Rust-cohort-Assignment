use std::io;
// standard input/output library.


struct Student {
    // its a structure that holds the data of students.
    name: String,
    score: i32,
    grade: String,
}

fn main() {
    let mut input:String = String:: new();
    // declaring a variable to hold user input and with a data type string. 

    println!("Enter the number of students in the class:");

    let no_students: i32;
    loop{
        io::stdin().read_line(&mut input).expect("Failed to read line");
        // read_line is a function that reads the input from the user
        // expect is a function that handles the error if the input is not valid
        
    
        
        match input.trim().parse::<i32>() {
            /*match - it compares the input that has been parsed(converted  to an integer) and if it matches the pattern,
            it  then returns the value, so the Ok or Err functions can handle errors.     */
            // trim is a function that removes the whitespace from the input.
            Ok(num) => {
                // Ok is afunction that outputs the value if the inputs have been validated i.e parsed and matched.
                if num >=10 {
                    // This allows or streamlines the input to b at least 10 students
                    no_students = num;
                    break;
                    // if its >= 10 the loop breaks and the rest of the code is executed.
                }
                else{
                    // this line of code runss if the number of students is less than 10.
                    println!("The number of students in the class must be at least 10");
                    input.clear();
                    // the input.clear(); allow the inputed values in the input variable to  be cleared if the input is less than 10.
                    continue;
                    // if the input is <10 the loop continues until d user inputs a value >=10.
                }
            }
            Err(_) => {
                // Err is a function that returns the error if the input is not validated.
                println!("Please enter a valid number");
                input.clear();
                continue;
                /*
                if the value isnt matched then its displays the "pls enter a valid number  and clears the input and continues running the loop."
                 */
            }
        }
    }


    let mut students: Vec<Student> = Vec::new();
    // vec=vector its a growable array in rust it can also shrink so therefore it can shrink.
    // the vector is a data structure that holds the data of the students.
    for i in 0..no_students {
        // a for loop that iterates from 0 to any amount of students that the user input that is >=10
        let mut name = String::new();// makes the name variable mutable, and creates space for the name of the student.
        let mut score = String::new();// makes the score variable mutable, and creates space for the score of the student.
        let  grade:String ; //makes the grade variable.

        println!("Enter the name of student {}: ", i + 1);
        // the i+1 and println! is used to display the data(name) of the student and also allows the iteration to be +1.
        io::stdin().read_line(&mut name).expect("Failed to read line");
        //allows to read user input, from a standard keyboard
        name = name.trim().to_string();
        // the above line of code trims away any whitespace that might have been inputed by the user while inputing value. 

        loop{
            println!("Enter the score of student {}: ", i + 1);
              // the i+1 and println! is used to display the data(score) of the student and also allows the iteration to be +1.
            io::stdin().read_line(&mut score).expect("Failed to read line");
            //allows to read user input, from a standard keyboard
            match score.trim().parse::<i32>() {
                 /*match - it compares the input that has been parsed(converted  to an integer) and if it matches the pattern,
            it  then returns the value, so the Ok or Err functions can handle errors.     */
            // trim is a function that removes the whitespace from the input.
                Ok(num) => {
                    // The condtion in the Ok function is executed if the values have been parsed and matched and the process has been validated.
                    if num >= 0 && num <= 100 {
                        // its a condition i.e if the score is >=0 and also <= 100 then trim every whitespace that the user might input with thwe values.
                        score = score.trim().to_string();
                        break;
                        // if the above condition is met then the loop breaks.
                    } else {
                        /*
                         if the score isn't >=0 and <=100 then display ,
                         "Please enter a valid score between 0 and 100" ,
                         and  clear the score continue the loop.
                         */
                        println!("Please enter a valid score between 0 and 100");
                        score.clear();
                        continue;
                    }
                }
                Err(_) => {
                    // This code is executed if the values isnt validated.
                    println!("Please enter a valid number");
                    score.clear();
                    continue;
                }
            }
            
        }
// USING IF-ELSE STATEMENT TO ASSIGN GRADES TO A PARTICULAR SCORE.
        if score.parse::<i32>().unwrap() >= 70 {
            grade = "A".to_string();
        } else if score.parse::<i32>().unwrap() >= 60 {
            grade = "B".to_string();
        } else if score.parse::<i32>().unwrap() >= 55 {
            grade = "C".to_string();
        } else if score.parse::<i32>().unwrap() >= 45 {
            grade = "D".to_string();
            } else if score.parse::<i32>().unwrap() >= 40 {
            grade = "E".to_string();
        } else {
            grade = "F".to_string();
        }

        students.push(Student {
            /*  the push is a method that takes one one value and append(add to the end) it
             to the vector there by increasing the size of this container*/
            name,
            score: score.parse::<i32>().unwrap(),
            // makes sure the valye inputed for score is an integer.
           grade,
        });
    }

    println!("\n\nStudent Name\tScore\tGrade");
    // prints out a Type of heading.
    println!("-------------------------------------");  
    // prints out a lined of dashes.
    for student in &students {
        // a for loop that loops through the data that is being outputed and prints them in the manner stated below.
        println!("{}\t\t{}\t{}", student.name, student.score, student.grade);
    }
}

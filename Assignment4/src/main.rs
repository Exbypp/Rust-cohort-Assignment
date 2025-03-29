use std::io;// standard library for input and output.
fn print_header() // function to display the calculator banner AND the menu for solving.
{
    
    println!(
        r#"

 ██████╗ █████╗ ██╗      ██████╗██╗   ██╗██╗      █████╗ ████████╗ ██████╗ ██████╗ 
██╔════╝██╔══██╗██║     ██╔════╝██║   ██║██║     ██╔══██╗╚══██╔══╝██╔═══██╗██╔══██╗
██║     ███████║██║     ██║     ██║   ██║██║     ███████║   ██║   ██║   ██║██████╔╝
██║     ██╔══██║██║     ██║     ██║   ██║██║     ██╔══██║   ██║   ██║   ██║██╔══██╗
╚██████╗██║  ██║███████╗╚██████╗╚██████╔╝███████╗██║  ██║   ██║   ╚██████╔╝██║  ██║
 ╚═════╝╚═╝  ╚═╝╚══════╝ ╚═════╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝
                                                                                           
        
_____________________________________________________________________________________
_____________________________________________________________________________________
_____________________________________________________________________________________
        
        


"#
        );
}


fn print_menu() {
    
        println!(
r#"
                     ███╗   ███╗███████╗███╗   ██╗██╗   ██╗
                     ████╗ ████║██╔════╝████╗  ██║██║   ██║
                     ██╔████╔██║█████╗  ██╔██╗ ██║██║   ██║
                     ██║╚██╔╝██║██╔══╝  ██║╚██╗██║██║   ██║
                     ██║ ╚═╝ ██║███████╗██║ ╚████║╚██████╔╝
                     ╚═╝     ╚═╝╚══════╝╚═╝  ╚═══╝ ╚═════╝ 


_____________________________________________________________________________________
_____________________________________________________________________________________



                      +---+------------------------------+
                      | . |  TYPE ANOPERATION TO PERFORM |  
                      +---+------------------------------+
                      | 1 |  ADDITION/ SUBTRACTION       |
                      | 2 |  MULTIPLICATION              |
                      | 3 |  DIVISION                    |
                      | 4 |  CGPA CALCULATOR             |
                      | 5 |  EXIT                        |
                      +---+------------------------------+

                                      
"#
);//the r#........."# allows to print multiple lines strings.

}

fn addition() {
    println!("\x1B[2J\x1B[1;1H");//function for clearing screen for another operation.
    print_header();//allows the header function to be printed with the addition function.

    let mut sum :f32 = 0.0;//declaring of variables.
    let mut input = String::new();//declaration of a variable that will allow for user input.


    println!("INPUT NUMBERS TO ADD, PRESS ENTER AFTER EACH NUMBER(WRITE NEGATIVE NUMBERS FOR SUBTRACTION), TYPE a non numerical to print answer");
   
    io::stdin().read_line(&mut input).expect("Failed to read line");
    //allows for user input from standard input ( keyboard).
    
  loop {
      
      // this loop performs the addition

        match input.trim().parse::<f32>() 
        /*the match compares and then joins value inputed with the variable(memory space) ,if the PARSED(Converted value)
        i.e from possible string to float) is successful
            The trim :trims or removes any trailing white space that the user will have inputed.
            inputs holds the user input that was read by the keyboard.
        */
        {
            Ok(num)=>{ /*the  'Ok and Err' is used for error handling  i.e->is used to make sure its the correct data type that has been inputed 
                              so if the correct data type is inputed the loop runs.
                */
                sum = sum + num;    //this line performs the addition.
                println!(" + ");    
            },
            Err(..)=>{ /*
                       if the wrong data type is inputed the loop breaks.
                 */
                break
            }
        }

    
        input.clear();//This lines clears the user input to allow for user input.
        io::stdin().read_line(&mut input).expect("Failed to read line");


        
    }
    
    println!("Final Result is: {}\n\n\n", sum);

    println!("Press return to continue...");

    io::stdin().read_line(&mut input).expect("Failed to read line");

}


fn multiplication() {
    println!("\x1B[2J\x1B[1;1H");//function for clearing screen for another operation.
    print_header();//allows the header function to be printed with the multiplication function.

    let mut product :f32 = 1.0;
    let mut input = String::new();// declaring of variable that allows for user input


    println!("INPUT NUMBERS TO MULTIPLYPress enter and EQUAL TO '=' to Get final result");
   
    io::stdin().read_line(&mut input).expect("Failed to read line");
    // allows for user input.
    

  loop {
      
        match input.trim().parse::<f32>()
          /*the match compares and then joins value inputed with the variable(memory space) ,if the PARSED(Converted value)
        i.e from possible string to float) is successful
            The trim :trims or removes any trailing white space that the user will have inputed.
            inputs holds the user input that was read by the keyboard.
        */
         {
            Ok(num)=>{
                /*the  'Ok and Err' is used for error handling  i.e->is used to make sure its the correct data type that has been inputed 
                              so if the correct data type is inputed the loop runs.
                */
                product = product * num;// does the multiplication.
                println!(" * ");
            },
            Err(..)=>{
                // if the wrong data is input its reads error and breaks the loop
                break
            }
        }

    
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
    }
    
    println!("Final product is: {}\n\n\n", product);

    println!("Press return to continue...");
    io::stdin().read_line(&mut input).expect("Failed to read line");

}

fn division() {

    println!("\x1B[2J\x1B[1;1H");//function for clearing screen for another operation.
    
    print_header();// allows the header function to be printed with the division function.

    let num1 :f32;
    let num2 :f32;
    let mut input = String::new();//declaring of user input variable.

    println!("Type The Two numbers to divide, a/b");

    println!("a: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    num1 = input.trim().parse().expect("Failed to read line");
    /*The trim :trims or removes any trailing white space that the user will have inputed.
            inputs holds the user input that was read by the keyboard.
            while parse converts the stored value to the desired variable.
            */
    input.clear();
    
    println!("b: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    // allows for user input.
    num2 = input.trim().parse().expect("Failed to read line");
 /*The trim :trims or removes any trailing white space that the user will have inputed.
            inputs holds the user input that was read by the keyboard.
            while parse converts the stored value to the desired variable.
            */
    println!("{}/{} = {}\n\n\n", num1, num2, num1/num2);

    println!("Press return to continue...");
    io::stdin().read_line(&mut input).expect("Failed to read line");

}
/*
 


fn cgpa_calculator(){
    println!("\x1B[2J\x1B[1;1H"); // Clear the screen for another operation.
    print_header(); // Display the header.

    let mut total_score: f32 = 0.0;
    let mut total_courses: u32 = 0;
    let mut input = String::new();

    println!("Enter scores for each course (0-100). Type 'done' to calculate CGPA.");

    loop {
        println!("Enter  course score {}: ", total_courses + 1);
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let trimmed_input = input.trim();

        if trimmed_input.eq_ignore_ascii_case("done") {
            break;
        }

        match trimmed_input.parse::<f32>() {
            Ok(score) if score >= 0.0 && score <= 100.0 => {
                total_score += score;
                total_courses += 1;
            }
            Ok(_) => {
                println!("Invalid score! Please enter a value between 0 and 100.");
            }
            Err(_) => {
                println!("Invalid input! Please enter a valid number or type 'done'.");
            }
        }

        input.clear();
    }

    if total_courses > 0 {
        let cgpa = total_score / total_courses as f32;
        println!("Your CGPA is: {:.2}\n\n", cgpa);
    } else {
        println!("No scores entered. CGPA cannot be calculated.\n\n");
    }

    println!("Press return to continue...");
    io::stdin().read_line(&mut input).expect("Failed to read line");
}
*/

fn main() {
  
    let mut input = String::new();
    

    loop {
        println!("\x1B[2J\x1B[1;1H");//function for clearing screen for another operation.
        print_header();//to print out the header function
        print_menu();// to print the menu.
        println!("Input Choice");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        //allows for user input.
       match input.trim().parse::<i32>(){
            Ok(num)=>{
                //Ok is to/The trim :trims or removes any trailing white space that the user will have inputed.
  // inputs holds the user input that was read by the keyboard. observer for erro
                match num {
                    //matches num to the following function names above.
                    1=>{addition()},
                    2=>{multiplication()},
                    3=>{division()},
                  //  4=>{cgpa_calculator()},
                    5=>{
                        break
                    },
                    _=>println!("invalid Choice"),}
            },
            Err(..)=>{
                // the is outpuyed if there is an error to the normal mode for the working calculator.
                println!("Failed to read line");
                input.clear();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                continue;
                // forces the processes to re-iterate.
            }
       }

        input.clear();
        
    }

    
}



fn main() {
    /*
                     ASSIGNMENT-3a        
    1. print out the 12 days of christmas song Using a loop.
    */
    println!("            MERRY CHRISTMAS 🎄🎄          ");
    println!("");
    let days=[ // an array stating all the 12 days.
      "FIrst",
      "Second",
      "Third",
      "Fourth",
      "Fifth",
      "sixth",
      "Seventh",
      "Eighth",
      "Ninth",
      "Tenth",
      "ELeventh",
      "Twelveth"
    ];
let giftlist=[ // An array stating the various gifts
"A patridge in a pear tree",
"Two turtle doves",
"Three french hens",
"Four calling birds",
"Five Golden rings",
"Six geese are laying",
"Seven swams are swimming",
"Eight maids are milking",
"Nine ladies dancing",
"Ten lords are leaping",
"Eleven pipers piping",
"Twelve drummers drumming",
];
    for each_day in 0..12{// using a for loop to iterate through the days 
        println!("On the {} day of christmas my true love sent to me",days[each_day]);
        //days[each_day] is used to access the days array.
        // each_day is used to access the index of the array getthe correct day.
        
        let gifts_range = 0..=each_day;  /*  declaring of the 0..=each_day scope. i.e if each_day=0 on d first day then the gift_range=0..=0
        The range represents the number of gifts recieved on a particular day+ all d gifts if the previous day.*/
     for gifts in gifts_range.rev(){
        // 0..=each_day is an inclusive range dat includes both 0 n also ends in each_day
        /*rev() is a reverse method.
         This allows the iteration to count down : (like from 12-1,instead of 1-12.) and restarts and also counts down again.*/
        // also declare the 0..=eachday range scope
        if each_day==0{
            // it cchecks if its the first day as each_day==0 indicates the first day.
            println!("{}",giftlist[gifts]);
            // if the each_day is 0 checks out as the first day, it prints the first gift on the giftlist array.
        }
        else if gifts==0{
            //checks if the first gift in the giftlist array, if it checks out.its prints the gift.
            //.to_lowercase()converts string to lowercase.
            println!("and {} ",giftlist[gifts].to_lowercase());
        }  
        else{
           print!("{}, ",giftlist[gifts]);
              // prints the gifts in the giftlist array.if its not the first day or first gift.

        }    
        println!("");// gives a line space after every iteration.
     }
    }

}

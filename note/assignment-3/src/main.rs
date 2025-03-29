fn main() {
    /*
                     ASSIGNMENT-3a        
    1. print out the 12 days of christmas song Using a loop.
    */
    println!("            MERRY CHRISTMAS 🎄🎄          ");
    println!("");
    let days=[ // an rray stating all the 12 days.
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
        let gifts_range = 0..=each_day;  // declaring of the 0..=each_day scope. 
     for gifts in gifts_range.rev(){
        // 0..=each_day is an inclusive range dat includes both 0 n also ends in eacd_day
        //rev() is a reverse method.
        // this allows the iteration to count down and restarts and also counts dow again.
        // also declare the 0..=eachday range scope
        if each_day==0{
            println!("{}",giftlist[gifts]);
        }
        else if gifts==0{
            //.to_lowercase()converts string to lowercase.
            println!("and {} ",giftlist[gifts].to_lowercase());
        }  
        else{
           print!("{}, ",giftlist[gifts]);

        }    
        println!("");
     }
    }

}

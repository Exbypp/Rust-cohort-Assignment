use std ::Fs::File;
// libary for calling file
/* mut number = 3;
while number !=0{
println!("{}",number);
number-=1;
}
let a =[20,10,15,10,9];
for element in a{
println!("The element in a are {}",element);
}


let fah=100;
let celcius=(5/9)*68;
if fah>=32{
println!("The temp is {} degree's C when you convert {} degree F ",celcius,fah);
}
let mut guess=string::new();
//io::stdin().read_line(&mut input).expect("Failed to readline"*/

fn open_file (filename:&str)->
Result<File,std::io::Error>{
    File::open(filename)
}    fn main(){
    match open_file("hello"),
    {
        Ok(file) => println!("File opened"),
        Err(error)>= println!("error opening file {:?}",error)
    }
}








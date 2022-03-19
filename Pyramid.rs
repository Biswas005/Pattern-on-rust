use std::io::stdin;

//function to take input from user
fn input() -> i32 {
//declare the varible as String to take as input
let mut number = String::new();
//read the varible and store it in predefined variable
stdin().read_line(&mut number)
       .ok()
       .expect("cannot read");
//convert the String varible into Integer of 32 bit
let number:i32 = number.trim()
                    .parse()
                    .ok()
                    .expect("Wrong entry cannot read");
//return value to the function it is called by
return number;
}

fn  main() {
    //declare the varible
    let mut _i:i32;
    let mut _j:i32;
    //calling the input function to take user defined input
    let row:i32 = input();
    let mut count:i32;

    count = row-1;
    for _i in 0..row
    {
        for _j in 0..count
        {
            print!(" ");
        }

        count = count-1;

        for _j in 0.._i+1
        {
            print!("* ");
        }
    println!();
    }

}

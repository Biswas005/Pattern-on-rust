use std::io::stdin;

fn input() -> i32 {
let mut number = String::new();
stdin().read_line(&mut number)
       .ok()
       .expect("cannot read");
let row:i32 = number.trim()
                    .parse()
                    .ok()
                    .expect("Wrong entry cannot read");
return row;
}

fn  main() {
    let mut _i:i32;
    let mut _j:i32;
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

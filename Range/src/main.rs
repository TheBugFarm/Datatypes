fn main() {
    
    let month_days = 1..=20;
    println!("{month_days:?}");

    for i in month_days {
        print!("{i} ");
    }

    println!();

    let letters = 'b'..'h';

    for i in letters {
        print!("{i} ");
    }
    
    println!();
     
    let colors = ["Grep", "nano", "cloud"];

    for i in colors {
        println!("{i} is a great color");  
    }
}

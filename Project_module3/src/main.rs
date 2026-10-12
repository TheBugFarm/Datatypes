fn main() {
    let yes: i32 = 1_337;
    println!("{yes}");

    let miles = yes as i16;
    println!("{miles}");

    let salt = 34.53736;
    println!("{salt:3}");

    let with_milk = true;
    let with_sugar = false;

    let is_my_type_of_coffee: bool;

    if with_sugar == true && with_milk == true {
        is_my_type_of_coffee = true;
    }
    let is_acceptable_coffee: bool;
    
    if with_sugar == true || with_milk == true {
        is_acceptable_coffee = true;
    }

    let r:[i8; 5] = [87, 96, 44, 22, 25];

    println!("{r:#?}");

    let p = (80085, 6.9, false, r);

    println!("{p:#?}");           
     
}


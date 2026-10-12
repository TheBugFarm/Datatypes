#[allow(unused_variables)]
fn main() {
    let r = ["greek", "latin", "kneel"];
    println!("the length of the array is {}", r.len());
    
    let george: [f64; 3] = [234.3, 234.4, 6.63354];

    let season: [&str; 4] = ["green", "latina", "russian", "brazilian"];

    let emp = ("navneet", 43, 'r');

    let name = emp.0;
    let age = emp.1;
    let alpha = emp.2;

    let (name, age, alpha) = emp;

    println!("{emp:#?}");


   
}
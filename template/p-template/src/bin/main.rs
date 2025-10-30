use {{ crate_name }}::solution::process;

fn main(){
    let file = include_str!("../input.txt");
    let result = process(file);
    println!("{result}");
}
use std::collections::HashMap;

#[derive(Debug)]
pub struct elf{
    calories: u32,
    num: u32,
}

fn main() {

    let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
    let elves = get_sorted_elves(input);
    print_top_n_elves(3, elves)
    // println!("{:?}", elf_to_ask);
    // println!("Elf: {:?}\nCalories: {:?}",elf_to_ask.0, elf_to_ask.1 );

}

pub fn print_top_n_elves(n: u32, mut e: Vec<elf>){
    for i in 0..n{
        let mut l = e.pop();
        println!("{:?}",l);
    }
}

pub fn get_sorted_elves(s: &str) -> Vec<elf>{
    let list = s.split("\n\n").collect::<Vec<&str>>();
    let mut counter = 0;

    let mut e_vec: Vec<elf> = Vec::new();

    for c in list{
        counter = counter + 1;
        let tmp : Vec<u32> = c.split('\n').map(|x| x.parse::<u32>().unwrap()).collect();
        let t_sum: u32 = tmp.iter().sum();
        let elf = elf{calories: t_sum, num : counter};
        e_vec.push(elf);
    }

    e_vec.sort_by_key(|e| e.calories);
    e_vec
}

fn remove_white_space(s: &str) -> String{
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

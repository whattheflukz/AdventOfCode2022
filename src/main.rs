use std::collections::HashMap;


fn main() {

    let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
    let elf_to_ask = get_best_elf(input);
    println!("Elf: {:?}\nCalories: {:?}",elf_to_ask.0, elf_to_ask.1 );

}

pub fn get_best_elf(s: &str) -> (u32,u32){
    let list = s.split("\n\n").collect::<Vec<&str>>();
    let mut elf_num = 0;
    let mut high = 0;
    let mut counter = 0;

    for c in list{
        counter = counter + 1;
        // if()
        let tmp : Vec<u32> = c.split('\n').map(|x| x.parse::<u32>().unwrap()).collect();
        let t_sum: u32 = tmp.iter().sum();
        if high < t_sum {
            high = t_sum;
            elf_num = counter;
        }
    }
    (elf_num, high)
}

fn remove_white_space(s: &str) -> String{
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
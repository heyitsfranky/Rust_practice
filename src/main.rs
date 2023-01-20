//#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
// Example 1
    println!("What is your name?");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you!";
    //io::stdin().read_line(&mut name).expect("Didn't receive input!");
    println!("Hello {}! {}", name.trim_end(), greeting);

// Example 2
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age += 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);

// Example 3
    println!("Max u32 : {}", u32::MAX);
    println!("Max u64 : {}", u64::MAX);
    println!("Max usize : {}", usize::MAX);
    println!("Max f64 : {}", f64::MAX);

// Example 4
    let is_true = true;
    let my_grade = 'A';

// Example 5
    let num_1: f32 = 1.111111111111111;
    println!("f32 : {}", num_1 + 0.111111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64 : {}", num_2 + 0.111111111111111);

// Example 6
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random : {}", random_num);

// Example 7
    let age = 8;
    if (age >= 1) && (age <= 18)
    {
        println!("Important Birthday!");
    } 
    else if age == 21 || age == 50
    {
        println!("Also important birthday!");
    }
    else
    {
        println!("Not an important birthday!");
    }

// Example 8
    let age2 = 8;
    match age2 
    {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday as well!"),
        65..=i32::MAX => println!("Important Birthday!"),
        _ => println!("Not an important birthday!"),
    }

// Example 9
    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age)
    {
        Ordering::Less => println!("Can't vote!"),
        Ordering::Greater => println!("Can vote!"),
        Ordering::Equal => println!("Can vote!")
    }

// Example 10
    let arr_1 = [1,2,3,4];
    println!("1st : {}", arr_1[0]);
    println!("Length : {}", arr_1.len());
    let arr_2 = [1,2,3];
    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0{
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 3{
            break;
        }
        println!("Val : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

// Example 11
    for val in arr_2.iter() {
        println!("Val : {}", val);
    }

// Example 12
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
    println!("Name : {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
    println!("Name : {}", v2);

// Example 13
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for word in st1.split_whitespace(){
        println!("{}", word);
    }
    let st2 = st1.replace('A', "Another");
    println!("{}", st2);

// Example 14
    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1{
        println!("{}", char);
    }
    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    st5.clear();
    let st8 = st3 + &st5; //st3 vanishes here, st5 stays
    


}

use std::io;
use rand::prelude::*;
fn main(){

    let fruit=["apple","banana","orange"];
    let mut rng=thread_rng();
    let index=rng.gen_rang(0..fruit.length());
    let random_fruit=fruits[index];
    println!("random fruit:{}",random_fruit);
    let mut input=String::new();
    match io::stdin().readline(&mut input){
        Ok(_)=>{
            let fruit_selected=input.trim().to_lowercase();
            println!("fruit selected:{}",fruit_selected);
        }
        err(error)=>{
            println!("error:{}",error);
        }
    }


    

}
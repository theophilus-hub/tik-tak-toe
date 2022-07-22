mod map;
use map::map;
use rand::prelude::*;
use std::io;
use std::mem::replace;


fn main() {
    let mut random: i32 = rand::thread_rng().gen_range(0..2);
    let mut coordinates: Vec<Vec<i32>> = vec![vec![0,0,0], vec![0,0,0], vec![0,0,0]];
    let mut game_choice: String = String::new();
    let mut user_input: String = String::new();
    let mut v: i32 = 0;
    let mut numpad: Vec<i32> = vec![1,2,3,4,5,6,7,8,9];


    //METHOD OF PLAY
    println!("how would you like to play? \nchoose between 1 and 2 \n (1)text? (2)numpad?");
    io::stdin().read_line(&mut game_choice).expect("cant read input");

        if game_choice.trim() == "1".to_string() {
            map(&coordinates);
            println!("\nyou can play using the main keywords top, middle and bottom in combinations \n e.g tm --top middle, bl ---- bottom left ");
        
        
        }else{
            map(&coordinates);
            println!("\nyou can play using the natural mapping of your numpad\n");


            loop{

                io::stdin().read_line(&mut user_input).expect("cant read input");
                if user_input.trim() == "".to_string(){
                    println!("you didnt enter any number!!!\n")
                }else{
                    v = user_input.trim().parse().unwrap();
                }
                for i in numpad.clone(){
                    if i == v {
                        //random number increase
                        random +=1;

                        if v == 1 {
                            let _change = replace(&mut coordinates[2][0], random);
                        }else if v == 2 {
                            let _change = replace(&mut coordinates[2][1], random);
                        }else if v == 3 {
                            let _change = replace(&mut coordinates[2][2], random);
                        }else if v == 4 {
                            let _change = replace(&mut coordinates[1][0], random);
                        }else if v == 5 {
                            let _change = replace(&mut coordinates[1][1], random);
                        }else if v == 6 {
                            let _change = replace(&mut coordinates[1][2], random);
                        }else if v == 7 {
                            let _change = replace(&mut coordinates[0][0], random);
                        }else if v == 8 {
                            let _change = replace(&mut coordinates[0][1], random);
                        }else{
                            let _change = replace(&mut coordinates[0][2], random);
                        }


                        numpad.retain(|&x| x != v);
                        map(&coordinates);
                        break;
                    }else{
                        if i == numpad[numpad.len()-1]{
                            print!("you can only play on empty spaces try again\n\n")
                        }
                    }
                }

                //println!("{:?}",coordinates );
                //println!("{:?}",numpad);
                if numpad.len().clone() == 0 {
                    break;
                }else{
                    user_input = "".to_string();
                }

            }
          

                
        }
    
    
  
}

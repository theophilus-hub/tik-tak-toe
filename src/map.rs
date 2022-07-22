pub fn map(coordinates: &Vec<Vec<i32>>){
   
    println!("TIK TAC TOE!");
    let rows: Vec<i32> =     vec![1,0,1,0,1,0,1];
    let coloumns: Vec<i32> = vec![1,0,1,0,1,0,1];
    let mut row_counter: usize = 0;
    let mut element_counter: usize = 0;
    let mut painter: String = String::new();
    let mut _box_position: String = String::new();

    for i in 0..rows.len(){
        if i%2 == 0 {
            for j in 0..coloumns.len(){
                if j%2 == 0 {
                    painter.push_str(". ");
                }else{
                    painter.push_str(". ");
                }
            }
        }else{
            for j in 0..coloumns.len(){
                if j%2 == 0 {
                    painter.push_str(". ");
                }else{
                    //main X AND O
                    if coordinates[row_counter][element_counter] == 0 {
                        painter.push_str("  ");
                    }else if coordinates[row_counter][element_counter]%2 == 1 {
                        painter.push_str("X ");
                    }else {
                        painter.push_str("O ");
                    }
                    if element_counter == 2{
                        element_counter = 0;
                    }else{
                        element_counter +=1;
                    }
                }
            }
                row_counter +=1;                
        }
        println!("\t{}",painter );
        painter = "".to_string();
    }

}
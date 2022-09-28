fn main(){
    draw_pyramid();
}

fn draw_pyramid(){
    let mut count:u8=0;
     loop{
        count+= 1;

        for _j in 0..count{
            print!{"{}",count}
        }
                println!{""}
                if count == 5{
                break;
            }
    }   
       
}
fn main(){

    let  numbers_x = vec![10, 20, 33, 46, 55];

    check_divisible_five(numbers_x);
}

fn check_divisible_five(ve:Vec<u8>){

     println!{"Given list: {:?}",ve};

    for number in ve{
        if number%5==0{
            println!{"{}",number};
        }else{
            
        }
        
    }
}
fn main(){

    let  numbers_x = vec![10, 20, 30, 40, 10];
    let numbers_y = vec![75, 65, 35, 75, 30];

    check_list(numbers_x);
    check_list(numbers_y);
}

fn check_list(ve:Vec<u8>){

     println!{"Given list: {:?}",ve};

    if ve[0]==ve[4]{
        println!{"result is True"};
    }else{
        println!{"result is false"};
    }
}
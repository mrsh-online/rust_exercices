fn main(){
    let vec1 = vec![10, 20, 25, 30, 35];
    let vec2 = vec![40, 45, 60, 75, 90];
    odd_vec(vec1,vec2);
}

fn odd_vec(vec1:Vec<i8>,vec2:Vec<i8>){
    let mut new_vec : Vec<i8> = Vec::new();

    for number in &vec1{
        if number % 2 == 0 {
            new_vec.push(*number);
        }else{
        
        }
    }

    for number in &vec2{
        if number % 2 == 0 {
            new_vec.push(*number);
        }else{
        
        }
    }

    println!("Vector 1 :{:?}", new_vec);

}

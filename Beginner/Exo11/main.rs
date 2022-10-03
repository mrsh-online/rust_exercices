fn main(){
    let number : i16 = 7536;
    reverse_number(number)
}

fn reverse_number(num:i16){
    let num_to_string :String = num.to_string();
    let reverse_string:String= num_to_string.chars().rev().collect();

    println!("{:?}",reverse_string);
}

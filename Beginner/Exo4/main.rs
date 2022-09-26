fn main(){
    remove_char("pynative", 4);
}

fn remove_char(phrase:&str, count:i8){
    let mut chars = phrase.chars();
    let mut iterator = 0;

    loop{
        chars.next();
        iterator += 1;

        if count == iterator{
            break;
        }
    } 
    println!("phrase : \"{}\" \ncount : {}",chars.as_str(), count)

}


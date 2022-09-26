fn main(){
    odd_phrase("pynative");
}

fn odd_phrase(phrase:&str){
    let mut count = 0;

    for c in phrase.chars(){

        if count & 1!=1{
            println!("{}",c);
        }
        count += 1;
    }

}


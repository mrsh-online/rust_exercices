fn main(){
    stars();

}

fn stars(){
    let mut counter = 6;
    for _i in 0..5{
        let mut stars = String::new();
        counter = counter-1;
        for _i in 0..counter{
            let single_star : char = '*';
            stars.push(single_star)
        }
        println!("{}", stars);
    }

}

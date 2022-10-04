fn main(){
    multiplication();
}

fn multiplication(){
    for table in 1..11{
       let mut array = Vec::new();
       
        for  multi in 1..11{
            array.push(multi * table)
        }
        println!{"\ntable de : {}\nresultat : {:?}",table, array}
    }
}

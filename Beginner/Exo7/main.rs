fn main(){

    let  str_x = "Emma is good developer. Emma is a writer";
    count_occurence(str_x);
}

fn count_occurence(s:&str){
    let count = s.matches("Emma").count();

     println!{"Emma appeared {} times",count};

     
}
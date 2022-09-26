fn main(){
    iteration(10);

}

fn iteration(num:i8){
    let mut count = 0;
    let mut current;
    let mut previous=0;
    let mut sum;

    println!{"Printing current and previous number sum in a range  {}", num};
    loop{
        current = count;
        sum = current +previous;
        println!{"Current number {} Previous number {} Sum {}",current,previous,sum };
        previous = current;
        count += 1;

        if count == num{
        break;
        }
    }


}


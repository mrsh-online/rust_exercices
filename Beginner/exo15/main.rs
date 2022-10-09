fn main(){
    exponent(5, 4);

}

fn exponent(base:i16,exp:i16){
    let mut i:i16 = 0;
    let mut result:i16 = base;
    while i < exp -1{
        result = base * result;
        i=i+1;
    }
    println!("base = {}\n", base);
    println!("exponent = {}\n", exp);
    print!("{} raises to the power of {}: {} i.e (", base,exp,result);
    let mut b = 0;
        while b < exp{
            if b ==0{
                print!("{} ", exp);
            }else{
                print!(" *{} ", exp);
            }
            b =b+1;
        } 
        print!(" = {})\n", base);
}

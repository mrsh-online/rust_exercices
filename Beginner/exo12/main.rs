fn main(){
    let income : f64= 45000.00;
    calculate_taxes(income);
   
}

fn calculate_taxes(income : f64){
    let taxe : f64=
        if income <= 10000.0 {
            0.0
        }else if income <= 20000.0 {
            let _x = income - 10000.0;
            _x * 0.1
        }else{
            let _x = 10000.0 * 0.1;
            _x + ((income-20000.0)*0.2) 
        };
   println!("taxe : {}", taxe);
}

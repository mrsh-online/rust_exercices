fn main() {
    multiply(20,30);
    multiply(40,30);
}

fn multiply(n1:i32,n2:i32){

   let mult = n1 * n2;

   if mult<1000 {
       println!("『multiply』The result is {}",{mult})
   }else{
    let sum = n1 + n2;
       println!("『sum』The result is {}",{sum})
       
   }
}

use std::{io::{Error,ErrorKind}, u128};

use bilzaa2danimation::Animation;
fn main(){
let a = Animation::new(1, 10, 0, 1000, "generator", "width");
println!("{:?}",a);
match a {
    Ok(b)=>{

        // println!("milli_per_pix{:?}",b.milli_per_pix());
        println!("at 1000 ms x ={:?}",b.animate(1000));
        println!("at 10,000 ms x ={:?}",b.animate(10000));

    },
    Err(e)=>(),
}
  
}


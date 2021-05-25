use std::{io::{Error,ErrorKind}, u128};

use bilzaa2danimation::Animation;
fn main(){
let a = Animation::new(1, 5, 0, 1000, "generator", "width");
println!("{:?}",a);
  
}


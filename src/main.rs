use std::{io::{Error,ErrorKind}, u128};

use bilzaa2danimation::Animation;
fn main(){
let a = Animation::new(100, 5, 100, 1000, "generator", "width");
println!("{:?}",a);
  
}


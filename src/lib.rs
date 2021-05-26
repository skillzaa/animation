
mod calc;
pub use calc::percent_to_value;
use std::{io::{Error,ErrorKind}, ops::Div, u128};
#[derive(Debug)]
pub struct Animation {
    from_time:u128,
    to_time:u128,
    from:u128,
    to:u128,
    is_reverse:bool, 
    animation_duration:u128,
    animation_distance:u128,
    generator:String,
    attr_to_animate:String,
}
//==========================================

impl Animation{
    pub fn new(from_time:u128,to_time:u128,from:u128,to:u128,generator:&str,attr_to_animate:&str)->Result<Animation,Error>{
    assert!(from_time < to_time,"From time can not be bigger than To time");   
    assert!(from < 5000,"From value is too large");
    assert!(to < 5000,"To value is too large");
        let from_time_millis:u128 = from_time * 1000;
        let to_time_millis:u128 = to_time * 1000;
            Ok(Animation {
                from_time : from_time_millis,
                to_time : to_time_millis,
                from,
                to,
                is_reverse: is_reverse(from, to),
                animation_duration : animation_duration(to_time_millis, from_time_millis), 
                animation_distance:animation_distance(from, to),
                generator:String::from(generator),
                attr_to_animate:String::from(attr_to_animate),
            })
    }       
    pub fn animate(&self,time_ms:u128)->u128{      
   88   
    }
    
    }//end of impl block
fn is_reverse(from:u128,to:u128)->bool{
    if from < to {
        false
    }else {
        true
    }
}
fn animation_duration(to_time:u128,from_time:u128)->u128{
    let ret:u128 = to_time - from_time;
    ret
}
fn animation_distance (from:u128,to:u128)->u128{
    let c = to - from as u128;
        c
}

///////////////////////////////===========tests
#[cfg(test)]
#[test]
fn basic(){
    let a = Animation::new(0, 10, 0, 100, "generator", "width");
    // println!("{:?}",a);
    // match a {
    //     Ok(b)=>{    
    //         // assert!(b.animate(0),0);
    //         // println!("at 0 ms x ={:?}",b.animate(1));
    //         // println!("at 1000 ms x ={:?}",b.animate(1000));
    //         println!("at 2500 ms x ={:?}",b.animate(3000));
    //         println!("at 5000 ms x ={:?}",b.animate(5000));
    //         assert!(true);
    
    //     },
    //     Err(e)=>(),
    // }
      
}
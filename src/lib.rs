
mod calc;
pub use calc::{percent_to_value,percentage};
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
    attr_to_animate:String,
}
//==========================================

impl Animation{
    pub fn new(from_time:u128,to_time:u128,from:u128,to:u128,attr_to_animate:&str)->Result<Animation,Error>{
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
                attr_to_animate:String::from(attr_to_animate),
            })
    } 
    fn is_time_valid(&self,time_ms:u128)->bool{
        if time_ms > self.to_time || time_ms < self.from_time {
            panic!("Time is not valid")
           }else {
               true
           }       
    }
    fn time_lapsed(&self,time_ms:u128)->u128{
        //--time is valid check is seperate.       
        time_ms - self.from_time
    }      
    pub fn animate(&self,time_ms:u128)->Option<u128>{     
        self.is_time_valid(time_ms);
        let time_lapsed = self.time_lapsed(time_ms);
        let time_perc_lapsed = percent_to_value(self.animation_duration as f64, time_lapsed as f64);
        match time_perc_lapsed {
            Some(x)=>{  
                //Some(x as u128)
                let per = percentage(self.animation_distance as f64,x);
                    match per {
                        Some(y)=> {
                            if self.is_reverse == true {
                                let f = y as u128 - self.to;
                                return Some(f)
                            }else {
                                let f = y as u128 + self.from;
                                return Some(f)
                            }
                        },
                        None=> return None,   
                    }
            },
            None=> return None,
        }
       
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

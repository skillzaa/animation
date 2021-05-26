use std::{io::{Error,ErrorKind}, u128};
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
        self.is_time_valid(time_ms);
        
        if self.is_reverse {
            return  self.milli_per_pix() * self.millis_lapsed(time_ms) - self.to;
        }else{
            return  self.milli_per_pix() * self.millis_lapsed(time_ms) + self.from;        }        
    }
        //-----private api
    pub fn milli_per_pix(&self)->u128{
        assert!(self.animation_distance > 0, "Distance cant be smaller than zero");   
        let c = self.animation_duration/self.animation_distance as u128;
        c
        }    
    fn is_time_valid(&self,time_ms:u128)->bool{
        if time_ms > self.to_time || time_ms < self.from_time {
            panic!("Time is not valid")
           }else {
               true
           }       
    }
    fn millis_lapsed(&self,time_ms:u128)->u128{
        //--time is valid check is seperate.       
        time_ms - self.from_time
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
fn test_is_time_valid() {
    let a = Animation::new(1,10,100,1000,"counter","width");
    match a {
        Ok(a)=>{
            for i in 1000..10000{
                assert!(a.is_time_valid(i));
            }    
        },        
        Err(e)=>(),
    }
}

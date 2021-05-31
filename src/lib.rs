
mod calc;
pub use calc::{percent_to_value,percentage};

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
    pub fn new(from_time:u128,to_time:u128,from:u128,to:u128,attr_to_animate:&str)->Option<Animation>{
    assert!(from_time < to_time,"From time can not be bigger than To time");   
    assert!(from < 5000,"From value is too large");
    assert!(to < 5000,"To value is too large");
        let from_time_millis:u128 = from_time * 1000;
        let to_time_millis:u128 = to_time * 1000;
            Some(Animation {
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
    pub fn is_time_valid(&self,time_ms:u128)->Option<bool>{
        if time_ms > self.to_time || time_ms < self.from_time {
            return None;
           }else {
               Some(true)
           }       
    }
    fn time_lapsed(&self,time_ms:u128)->u128{
        //--time is valid check is seperate.       
        time_ms - self.from_time
    }      
    pub fn animate(&self,time_ms:u128)->Option<u128>{     
        self.is_time_valid(time_ms)?;
        let time_lapsed = self.time_lapsed(time_ms);
        let time_perc_lapsed = percent_to_value(self.animation_duration as f64, time_lapsed as f64);
        match time_perc_lapsed {
            Some(x)=>{  
                let per:Option<u128> = percentage(self.animation_distance as f64,x);
                    match per {
                        Some(y)=> {
                            if self.is_reverse == true {
                               return Some(self.from - y)
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
   
    pub fn get_attr_to_animate(&self)->String{
        String::from(&self.attr_to_animate)
    }    
}//end of impl block
fn is_reverse(from:u128,to:u128)->bool{
    if from < to {
        return false
    }else {
        return true
    }
}
fn animation_duration(to_time:u128,from_time:u128)->u128{
    if from_time > to_time {
        return from_time - to_time
    }else {
        return to_time - from_time
    }
}
fn animation_distance (from:u128,to:u128)->u128{
    if from > to {
        return from - to
    }else {
        return to - from
    }
}

//----public API


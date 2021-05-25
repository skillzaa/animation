use std::{io::{Error,ErrorKind}, u128};
#[derive(Debug)]
pub struct Animation {
    from_time:u128,
    to_time:u128,
    from:u128,
    to:u128,
    generator:String,
    attr_to_animate:String,
}
//==========================================

impl Animation{
    pub fn new(from_time:u128,to_time:u128,from:u128,to:u128,generator:&str,attr_to_animate:&str)->Result<Animation,Error>{
    assert!(from_time < to_time,"From time can not be bigger than To time");   
    assert!(from < 5000,"From value is too large");
    assert!(to < 5000,"To value is too large");

            Ok(Animation {
                from_time : from_time * 1000,
                to_time : to_time * 1000,
                from,
                to,
                generator:String::from(generator),
                attr_to_animate:String::from(attr_to_animate),
            })
    }
       
    pub fn animate(&self,time_ms:u128)->Result<u128,Error>{   
    
        self.is_time_valid(time_ms)?;
        if self.is_reverse() {
            let milli_per_pix = self.milli_per_pix()?;
            let time_lapsed = self.time_lapsed(time_ms);
            return Ok( milli_per_pix * time_lapsed - self.from);
        }else{
            let milli_per_pix = self.milli_per_pix()?;
            let time_lapsed = self.time_lapsed(time_ms);
            return Ok( milli_per_pix * time_lapsed + self.from);
        }        
    }
    //-----private api
    fn is_time_valid(&self,time_ms:u128)->Result<bool,Error>{
        if time_ms > self.to_time || time_ms < self.from_time {
            Err(Error::new(ErrorKind::Other, "Time is not valid"))
           }else {
               Ok(true)
           }       
    }
    fn time_lapsed(&self,time_ms:u128)->u128{
        //--time is valid check is seperate.       
        time_ms - self.from_time
    }
    fn animation_duration(&self)->u128{
        self.to_time - self.from_time
    }
    fn animation_distance (&self)->u128{
        self.to - self.from
    }
    fn milli_per_pix(&self)->Result<u128,Error>{
        let a = self.animation_duration();
        let b = self.animation_distance();
        if b == 0 {
           Err(Error::new(ErrorKind::Other, "Division By Zero"))
          }else {
            let c = a/b as u128;
            Ok(c)
          }
    }
    fn is_reverse(&self)->bool{
        if self.from < self.to {
            false
        }else {
            true
        }
    }

}//end of impl block
///////////////////////////////===========tests
#[cfg(test)]
#[test]
fn test_is_time_valid() {
    let a = Animation::new(1,10,100,1000,"counter","width");
    match a {
        Ok(a)=>{
            for i in 1000..10000{
                match a.is_time_valid(i) {
                    Ok(v)=> assert_eq!(v,true),
                    Err(e)=>{
                        panic!("it should never come to this");
                    },
                }
            }
        },        
        Err(e)=>(),
    }
    // println!("{:?}",b);
}
#[cfg(test)]
#[test]
fn test_is_time_valid2() {
    let a = Animation::new(1,10,100,1000,"counter","width");
    match a {
        Ok(a)=>{
            for i in 1..999{
                match a.is_time_valid(i) {
                    Ok(v)=> panic!("Should not come to this"),
                    Err(e)=>{
                        assert_eq!(std::io::ErrorKind::Other,e.kind());
                        // println!("{:?}",e.kind());
                    },
                }
            }
        },        
        Err(e)=>(),
    }
    // println!("{:?}",b);
}

use bilzaa2danimation::Animation;

#[cfg(test)]
#[test]
fn basic_must_first(){
    let a = Animation::new(0, 10, 0, 100, "width");
    println!("{:?}",a);
    match a {
        Ok(b)=>{    
            
            match b.animate(1000_u128) {
                Some(x)=> assert_eq!(x as u128,10),
                None=> panic!("Panic"),
            }
            match b.animate(5000_u128) {
                Some(x)=> assert_eq!(x as u128,50),
                None=> panic!("Panic"),
            }
            match b.animate(10000_u128) {
                Some(x)=> assert_eq!(x as u128,100),
                None=> panic!("Panic"),
            }
    
        },
        Err(e)=>(),
    }
    
}
//////////////////////////////////
#[cfg(test)]
#[test]
fn basic_must_second(){
    let a = Animation::new(1, 21, 200, 400, "width");
    println!("{:?}",a);
    match a {
        Ok(b)=>{               
            match b.animate(11000_u128) {
                Some(x)=> assert_eq!(x as u128,300),
                None=> panic!("Panic"),
            }
            match b.animate(21000_u128) {
                Some(x)=> assert_eq!(x as u128,400),
                None=> panic!("Panic"),
            }   
        },
        Err(e)=>(),
    }    
}
#[cfg(test)]
#[test]
#[should_panic]
fn must_panic(){
    let a = Animation::new(1, 21, 200, 400, "width");
    println!("{:?}",a);
    match a {
        Ok(b)=>{               
            match b.animate(1000_u128) {
                //it should return none
                Some(x)=> (),
                None=>  panic!("Panic"),
            }
             
        },
        Err(e)=>(),
    }    
}
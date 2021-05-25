use bilzaa2danimation::Animation;
#[cfg(test)]
#[test]
fn first() {
    let a = Animation::new(1,10,100,1000,"counter","width");
    match a {
        Ok(a)=>{
            for i in 1000..10000{
                match a.animate(i) {
                    Ok(v)=>(),
                    Err(e)=>{
                        panic!("it should never come to this");
                    },
                }
            }
        },        
        Err(e)=>(),
    }
}

#[test]
fn second(){
    let a = get_ani();
    // assert!(a::t)
    println!("{:?}",a);
    }
fn get_ani()->u128{
555
}
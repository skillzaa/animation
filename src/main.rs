
use bilzaa2danimation::Animation;
fn main(){
    let a = Animation::new(0, 10, 0, 100, "width");
    println!("{:?}",a);
    match a {
        Ok(b)=>{    
           // println!("at 0 ms x ={:?}",b.animate(0));
           
            println!("at 1000 ms x ={:?}",b.animate(1000));
            println!("at 5,000 ms x ={:?}",b.animate(5000));
            println!("at 10,000 ms x ={:?}",b.animate(10000));
    
        },
        Err(e)=>(),
    }
    
}
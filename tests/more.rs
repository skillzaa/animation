use bilzaa2dcounter::Animation;
use bilzaa2dattributes::AttributesEnum;

//use #[should_panic] with the test --if to check errors 
fn test_a(a:Animation,time_ms:u128,answer:u128){
    match a.animate(time_ms) {
        Some(x)=> assert_eq!(x as u128,answer),
        None=> panic!("Panic"),
    }
}
fn get_a(from_time:u128,to_time:u128,from:u128,to:u128)->Animation{
    let a = Animation::new(from_time, to_time, from, to, AttributesEnum::Width);
    match a {
        Some(b)=>{return b},
        None=>panic!("Animation creation error"),
}
}
//////////////////////////////////
// #[cfg(test)]
#[test]
fn bullet_proof01(){
  let a:Animation = get_a(0,10,0,100); 
 test_a(a, 5000, 50);
}

#[test]
fn bullet_proof02(){
  let a:Animation = get_a(0,10,100,0); 
 test_a(a, 5000, 50);
}
#[test]
fn bullet_proof03(){
  let a:Animation = get_a(0,10,0,1000); 
 test_a(a, 5000, 500);
}


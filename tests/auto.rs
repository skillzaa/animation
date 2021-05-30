use bilzaa2dcounter::Animation;
//use #[should_panic] with the test --if to check errors 
fn test_a(a:Animation,time_ms:u128,answer:u128){
    match a.animate(time_ms) {
        Some(x)=> assert_eq!(x as u128,answer),
        None=> panic!("Panic"),
    }
}
fn get_a(from_time:u128,to_time:u128,from:u128,to:u128)->Animation{
    let a = Animation::new(from_time, to_time, from, to, "width");
    match a {
        Some(b)=>{return b},
        None=>panic!("Animation creation error"),
}
}
//////////////////////////////////
// #[cfg(test)]
#[test]
fn get_attr(){
  let a:Animation = get_a(0,10,0,100); 
  let ata = a.get_attr_to_animate();
 assert_eq!(ata,"width");
}
#[test]
fn first(){
  let a:Animation = get_a(0,10,0,100);  
 test_a(a, 10000, 100);
}
#[test]
fn mm(){
  let a:Animation = get_a(0,10,0,100);  
 test_a(a, 5000, 50);
}
#[test]
#[should_panic] // time is not right it shd be 15000
fn mmm(){
  let a:Animation = get_a(10,20,0,100);  
 test_a(a, 15, 50);
}
#[test]
#[should_panic]
fn auto_second(){
  let a:Animation = get_a(0,10,0,100);  
 test_a(a, 0, 0);
}
#[test]
fn aa(){
  let a:Animation = get_a(10,20,0,100);  
 test_a(a, 15000, 50);
}
#[test]
fn aaa(){
  let a:Animation = get_a(10,20,100,200);  
 test_a(a, 15000, 150);
}
#[test]
fn reverse_one(){
  let a:Animation = get_a(10,20,200,100);  
 test_a(a, 15000, 150);
}
#[test]
fn reverse_two(){
  let a:Animation = get_a(10,20,200,100);  
 test_a(a, 20000, 100);
}
#[test]
#[should_panic] // time is not vlaid
fn reverse_three(){ // its zero
  let a:Animation = get_a(10,20,200,100);  
 test_a(a, 0, 200);
}
#[test]
#[should_panic]
fn reverse_four(){ // its zero--start time--will ret none
  let a:Animation = get_a(10,20,200,100);  
 test_a(a, 10000, 200);
}
#[test]
fn reverse_five(){//50%
  let a:Animation = get_a(10,50,200,100);  
 test_a(a, 30000, 150);
}
#[test]
fn reverse_six(){//its 175 not 125 its reverse
  let a:Animation = get_a(10,50,200,100);  
 test_a(a, 20000, 175);
}
#[test]
fn normal(){//its 125 now since not reverse
  let a:Animation = get_a(10,50,100,200);  
 test_a(a, 20000, 125);
}
#[test]
fn normal_two(){//end
  let a:Animation = get_a(10,50,100,200);  
 test_a(a, 50000, 200);
}
#[test]
#[should_panic]
fn normal_three(){//end
  let a:Animation = get_a(10,50,100,200);  
 test_a(a, 10000, 100);
}
#[test]
#[should_panic]
fn time_wrong(){//end
  let a:Animation = get_a(10,5,100,200);  
 test_a(a, 10000, 100);
}
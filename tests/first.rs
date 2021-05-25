use bilzaa2danimation::Animation;

#[cfg(test)]
#[test]
fn basic_must(){
    let a = Animation::new(1, 5, 100, 1000, "generator", "width");
println!("{:?}",a);
}
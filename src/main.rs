fn main() {
    println!("Hello, world!");
    let ye =String::from("hello");
    let lee="123566777";
    fn big<'a>(x: &'a str,y: &'a str)->&'a str{
        if x.len() > y.len(){
            x
        }else{
        y
        }
    }
    println!("{}",big(ye.as_str(),lee))
}

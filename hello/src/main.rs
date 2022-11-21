fn main() {
    let x: Option<i32> = Some(3);
    println!("Hello, world!");
    
    let Some(x): &Option<i32> = &x else { return };
    println!("{}", x);
    
    named_fun(55);
}

fn named_fun(_named: i32) {
    
}

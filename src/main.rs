fn main()
 {
    let age: f32;
    age = 2.0;
    if age > 5.0{
        println!("age = {}", age)
    }
    else{
        println!("kaka");
    }
    matchh(age);
    //if let
    let qwer: i32 = if age > 5.0
    {
        5   
    } 
    else{
        10
    };
    println!("{}", qwer);
    //loops();
}

fn summ(){
    let mut x: i32 = 5;
    println!("Значение x: {}", x);
    x = 6;
    println!("Новое значение x: {}", x);
}

fn matchh(age:f32){
    match age{
         1.0 => println!("age = {}", age),
         2.0 => summ(),
        _=>println!("хз")
    }
    let result: &'static str = match age {
        1.0 => "один",
        2.0 => "два",
        _ => "хз"
    };
    println!("{}", result);
}
fn loops(){
    let mut days: i32 = 0;
    let mut check: bool = false;
    loop {
        days = days + 1;
        if days == 100 && check == false{
            days = 0;
            check = true;
        }
        else if days == 100 && check == true{
            break;
        }
        println!("days = {}", days);
    }
}
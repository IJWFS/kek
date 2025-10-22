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
    for i in 1..11
    {
        for j in 1..11
        {
            print!("{}\t", i * j);
        }
        println!();
        
    }
    // square(5);
    // square(10);
    // square(2);
    // square(99);
    let check: i32 = chec();
    println!("{}", check);

    //анонимные функции 
    let square_anon = |n: i32|{
        let result = n * n;
        println!("Квадрат числа {} равен {}", n, result);
    };
    square_anon(5);

    let block = {
        println!("kek");
        println!("kek");
        println!("kek");
        println!("kek");
        99
    };
    println!("block = {block}");



    let tuple: (&'static str, i32, f64) = ("kek", 1, 1.5);
    println!("tuple 0: {}", tuple.0);
    let mut kek: &'static str = tuple.0;
    print!("{}", kek);
    kek = "keks";

    let num:[i32; 3] = [1, 2, 1];
    for i in 0..num.len()
    {
        println!("{}", num[i]);
    }


    let mut users = ["Tom", "Bob", "Sam"];
    users.sort();
    println!("users: {:?} ", users);
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

fn square(n: i32)
{
    let result: i32 = n * n;
    println!("Квадрат числа {} равен {}", n, result);
}

fn chec() -> i32
{
    5
}
fn loops(){
    let mut days: i32 = 0;
    let mut check: bool = false;
    loop 
    {
        days = days + 1;
        if days == 100 && check == false
        {
            days = 0;
            check = true;
        }
        else if days == 100 && check == true
        {
            break;
        }
        println!("while days = {}", days);
    }
    days = 0;
    while days < 100
    {
        days = days + 1;
        println!("for days = {}", days);
    }

    for i in 0..10
    {
        println!("i = {}", i);
    }



}
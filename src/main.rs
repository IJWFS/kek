struct Person
{
    name: String,
    age: u8,
    height: f32
}


fn main() {
    let mut Tom = Person{
        name: "Tom".to_string(),
        age: 22,
        height: 180.0
    };
    println!("name = {}  age = {} height = {}", Tom.name, Tom.age, Tom.height);
    Tom.age = 23;
    Tom.height = 130.4;
    println!("name = {}  age = {} height = {}", Tom.name, Tom.age, Tom.height);

    println!("Hello, world!");


    let bob = Person{
        name: "Bob".to_string(),
        age: 33,
        height: 1.70
    };
    let tom = Person{
        name: "Tom".to_string(),
        ..bob // пополнение из bob
    };
    //запись в переменные
    println!("name = {}  age = {} height = {}", tom.name, tom.age, tom.height);
    let Person{name: username, age: userage, height: _} = tom;
    println!("name = {}  age = {}", username, userage);


    

}

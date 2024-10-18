// struct User{
//     name : String,
//     age : u32,
//     active : bool,
// }

// struct Numm {
//     width : u32,
//     hight : u32,
// }

// impl Numm {
//     fn area(&self) -> u32{
//         return self.width * self.hight;
//     }
// }

enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64,f64),
}

fn cal_area(shape: Shape) -> f64 {
    let ans = match shape{
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Square(side) =>side * side,
        Shape::Rectangle(hight,width) =>{hight * width},
    };
    return ans;
}

// enum Moves {
//     up,
//     down,
//     left,
//     right,
// }

fn main() {
    // println!("Hello, world!");

    //number

    // let x: i32 = 5;
    // let y: u32 = 10;
    // let z: f32 = 3.14;
    // print!("x = {}, y = {}, z = {}", x, y, z);

    //booleans
    // let x: bool = true;

    //strings
    // let my_string = String::from("Hello, world!");
    // print!("{}", my_string);

    //if-else
    // let x: i32= 5;
    // if x > 0{
    //   print!("number is bigger than 0");

    // }
    // else if x==0 {
    //   print!("number is 0");
    // }
    // else{
    //   print!("number is smaller than 0");
    // }

    // loops
    // let mut x: i32 = 5;
    // for i in 0..10 {
    //     print!("{}", i);
    //     println!();
    //     x = x + 1;
    //     print!("{}", x);
    //     println!();
    // }

    // function

    // let x: i32 = add(5, 10);
    // fn add(x: i32, y: i32) -> i32 {
    //     return x + y;
    // }
    // print!("value of x is {}", x);


    //memory mangement
    //stack and heap

    // stack_fn();
    // heap_fn();
    // update_string();


    // ownership

    // let s1 = String::from("priyanshu");
    // let s2 = s1;

    // println!("{}",s1);

    // let my_string = String::from("my string");
    // passto_other(my_string.clone());
    // println!("{}",my_string);

    // let mut my_string = String::from("my string");
    // my_string= passto_other(my_string);
    // println!("{}",my_string);

    //refances
    // let s1 = String::from("Hello");
    // let s2 = &s1;
    
    // println!("{}",s1);
    // println!("{}",s2);

    // borrowing
    // let my_string = String::from("hello");
    // borrow_val(&my_string);
    // println!("{}",my_string);

    // let mut s1 = String::from("hello");
    // update_str(&mut s1);
    // println!("{}",s1);

    // struct

    // let user1 = User {
    //     name : String::from("Priyanshu"),
    //     age : 30,
    //     active : true,
    // };
    // println!("{} 's age is {} and he is active : {}",user1.name,user1.age,user1.active);

    // implementation

    // let user1 = Numm {
    //     width : 30,
    //     hight : 30,
    // };

    // println!("with use of width : {} and hight: {} the area is {}",user1.width,user1.hight,user1.area());

    // Enums

    // move_around(Moves::down);

    //pattern matching

    let circle = Shape::Circle(5.0);
    let square = Shape::Square(5.0);
    let rectangle = Shape::Rectangle(5.0, 5.0);

    let area_circle = cal_area(circle);
    let area_square = cal_area(square);
    let area_rec = cal_area(rectangle);
    println!("{}",area_circle);
    println!("{}",area_square);
    println!("{}",area_rec);




}

// fn move_around (my_move:Moves)
// {
//     //your logic
// }


// fn update_str(str : &mut String)
// {
//     println!("priyanshu");
//     str.push_str("world"); 
//     println!("{}",str);
// }


// fn borrow_val(some_str : &String)
// {
//     println!("{}",some_str);
// }

// fn passto_other(some_string:String) -> String
// {
//     println!("{}",some_string);
//     return some_string;
// }

// fn stack_fn(){
//     let a:i32 = 10;
//     let b:i32 = 5;
//     let c:i32 = a+b;
//     println!("the sum of {} and {} is {}",a,b,c);
// }

// fn heap_fn()
// {
//     let s1 = String::from("Priyanshu");
//     let s2 = String::from("Patel");
//     let combined = format!("{} {}",s1,s2);
//     println!("the combination of {} and {} is {}",s1,s2,combined);
// }

// fn update_string()
// {
//     let mut s = String::from("Priyanshu");
//     println!("before update: {}",s);
//     println!("length {} , capacity {} , pointer{:p}",s.len(),s.capacity(),s.as_ptr());

//     s.push_str("Patel");

//     println!("After update: {}",s);
//     println!("length {} , capacity {} , pointer{:p}",s.len(),s.capacity(),s.as_ptr());
// }

// 3:25
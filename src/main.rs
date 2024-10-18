// use std::collections::HashMap;

// pub trait Myfun {
//     fn myfun(&self) -> String;
// }
// struct User{
//     name:String,
//     age :i32,
// }
// impl Myfun for User {
//     fn myfun(&self) -> String {
//         return format!("User {} is {} years old",self.name,self.age);
//     }
// }

fn main() {
    //collections
    //vector

    // let number = vec![1, 2, 3];

    // let mut vet = Vec::new();
    // vet.push(6);
    // vet.push(21);
    // vet.push(1);
    // vet.push(2);

    // let ans = even_filt(&mut vet);
    // even_filt(&mut vet);
    // print!("{:?}", vet);

    // print!("{:?}", even_filt(vet));

    //hash Map
    // let mut my_hm = HashMap::new();
    // my_hm.insert("priyanshu", 21);
    // my_hm.insert("nonee", 22);

    // let ans = my_hm.get("priyanshu"); //option<32>
    // match ans {
    //     Some(age) => print!("your age is {}", age),
    //     None => print!("none found"),
    // }

    // let  vect = vec![(String::from("priyanshu"), 22), (String::from("nonn"), 22)];

    // let hm = group_value_key(vect);
    // print!("{:?}", hm);

    //iterators

    //immutable iterators
    // let v1 = vec![1, 2, 3];
    // let iv = v1.iter();
    // for i in iv {
    //     print!("{}", i);
    // }

    //mutables iterators
    // let mut v1 = vec![1, 2, 3];
    // let iv = v1.iter_mut();
    // for i in iv {
    //     *i = *i + 1;
    //     print!("{}", i);
    // }

    //iterators with map
    // let v1 = vec![1, 2, 3];
    // let v2 = v1.iter();
    // let iter_v2 = v2.map(|x| x + 1);
    // for i in iter_v2 {
    //     print!("{}", i);
    // }

    //filter
    // let v1 = vec![1, 2, 3];
    // let v2 = v1.iter();
    // let iter_v2 = v2.filter(|x| *x % 2 == 0);
    // for i in iter_v2 {
    //     print!("{}", i);
    // }

    //string and slice
    // let word = String::from("priyanshu patel");
    // let ans = &word[0..5];
    // println!("{}", ans);

    // let vec = vec![1, 2, 3];
    // println!("{:?}", &vec[1..2]);

    //generics

    // let bigger = largest(1, 2);
    // let bigger_char = largest('a', 'b');
    // println!("{}", bigger);
    // println!("{}", bigger_char);

    //traits
    // let user = User {
    //     name : String::from("priyanshu"),
    //     age : 21,
    // };
    // println!("{}",user.myfun());

    //lifetimes
    let ans;
    let str1 = String::from("priyanshu");
    {
        let str2 = String::from("patel");
        ans = longest(&str1, &str2);
        
    }
    println!("{}", ans);
    
    
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        return str1;
    }
    return str2;
}

// fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
//     if a > b {
//         a
//     } else {
//         b
//     }
// }

// fn even_filt(vect: Vec<i32>) -> Vec<i32> {
//     let mut new_vec = Vec::new();
//     for val in vect {
//         if val % 2 == 0 {
//             new_vec.push(val);
//         }
//     }
//     return new_vec;
// }

// fn even_filt(vect: &mut Vec<i32>) {
//     let mut i = 0;
//     while i < vect.len() {
//         if vect[i] % 2 != 0 {
//             vect.remove(i);
//         } else {
//             i = i + 1;
//         }
//     }
// }

// fn group_value_key(my_vec: Vec<(String, i32)>) -> HashMap<String, i32> {
//     let mut hm = HashMap::new();
//     for (key, value) in my_vec {
//         hm.insert(key, value);
//     }
//     return hm;
// }

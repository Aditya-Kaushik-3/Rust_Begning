// use std::string;

// use std::io;
// use rand::Rag;

fn main(){
    // let _n: f64= 10.22;
    // let mut x: i64 = 12_124;
    // let t:&str = "false";
    // x = 2;
    // println!("hello");
    // println!("{}", 10);
    // print!("{}", x);

    // let seven = x as i64;
    // print!("{}", std::mem::size_of_val(&seven));
    // x+=1;
    // print!("{}",t);

    // let mut i = String::new();
    // i.push_str("hello");
    // let j = String::from("fuck");
    // println!("{}", i);
    // print!("{}", j);
    // let s = "cool".to_string();
    // let y = "brese".to_string();
    // let _zz = &s[1..4];
    // // let _yy = s+&y;

    // let _add = format!("{}{}",s,y);

    // let mut one = "xx xxx".to_string();
    // one.len();
    // // let _x = one.replace("xx", "xxx");
    // one.push('s');// for char
    // one.push_str("karm");
    // print!("{}",one); 
    // println!("hello \nWortld");
    // if 12 == 13{
    //     print!("ok");
    // }
    // else{print!("fuck")}

    // Match
    // let num = 10;
    // match num{
    //     1 => print!("one"),
    //     2..=10 => print!("Good"),
    //     _ =>print!("ok"),
    // }

    // let mut i = 0;
    // while i <= 10{
    //     print!("{}",i);
    //     i+=1;
    // }
    // loop{
    //     print!("ok");
    //     if i == 13{ break;}
    //     i+=1;
    // }
    // for x in (1..=10).rev(){
    //     println!("{}",x)
    // }
    // let mut line = String::new();

    // let l1 = std::io::stdin().read_line(&mut line).unwrap();
    // println!("{}",line);
    // let _x: i32 = line.trim().parse().expect("input INT");
    // println!("{}",_x);

    // let mut line = String::new();
    // let l2 = std::io::stdin().read_line(&mut line).unwrap();
    // println!("{}",line);
    // let _y: i32 = line.trim().parse().expect("input INT");
    // println!("{}",_y);

    // println!("The ans is {_x} {_y} {}",(_x+_y));

// Tuple
    // let a:(i128,i16,bool) = (21212,12,true);
    // print!("{:?}",a);
    // print!("{:?}",a.0);

    //Array
        // declare array
        // let arr = [100.4, 200.34];
        // println!("{:?}", arr);
        // // data size
        // let arr1: [i32; 4] = [1, 3, 4, 5];
        // println!("{:?}", arr1);
        // println!("{:?}", arr1.len());
        // // default value
        // let arr2: [i32; 10] = [100; 10];
        // // arr2[2] = 12;
        // println!("{:?}", arr2);
    
        // // mutable array
        // let mut arr3: [i32; 10] = [100; 10];
        // arr3[5] = 12;
        // println!("{:?}", arr3);

        // let arr = [1, 2, 4, 4, 6, 4, 3, 23, 45, 33, 3];
        // for i in arr.iter(){
        //     print!("{:?}", i);
        // }
        // for i in 1..11{
        //     println!("{:?}", arr[i]);
        // }

    // let vec:[[i32;2]; 4] = [[100, 101], [102, 103], [104, 105], [106, 107]];
    // println!("{:?}", vec.len());
    // println!("{:?}", vec[0][1]);


    //    let mut v= vec![12,34,56];
    // // v.push(100);// You can only change index values not increase the size
    // for i in v{
    //     println!("{}",i);
    // }
    // let mut v1:Vec<i32> = Vec::new();
    // v1.push(12);
    // v1.push(13);
    // println!("{}",v1[0]); 

    // print!("{}", add(10,11));

    // let v = vec![1, 2, 3, 4, 5];
    // println!("{:?}", v);
    // _cust(&v);
    // println!("{:?}", v);
    // let x = |i: i32| -> i32 { i * 10 };
    // println!("{}", x(12));
    // let y = |i| i * 100;
    // println!("{}", y(12));


       let mut employee_1 = Employee {
        name: String::from("Abhishek"),
        age: 17,
    };
    println!("Name {}", employee_1.name);
    println!("Age {}", employee_1.age);

    employee_1.age = 18;
    println!("Age {}", employee_1.age);

    input_struct(employee_1);

}
// fn add(num1:i32, num2:i32)->i32{
//     num1+num2
// }
// fn _cust(s: &Vec<i32>) {
//     println!("{:?}", s);
// }
struct Employee {
    name: String,
    age: i32,
}

fn input_struct(x: Employee) {
    println!("fun {}", x.name);
    println!("fun {}", x.age);

    add(10i8.into());
}
fn factorial(x: i32) -> i32 {
    if x > 1 {
        x * factorial(x - 1)
    } else {
        x
    }
}
fn add(x:i16){
    println!("{}",x);
}
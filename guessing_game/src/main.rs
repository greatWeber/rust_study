use std::io;
use std::cmp::Ordering;
use rand::Rng;
// 猜数字
fn main() {
    // guess_number();
    
    // let str = String::from("hello world");
    // let sub = slice_string(&str);
    // println!("sub: {}", sub);
    // println!("str: {}", str);

    // struct_object();

    // impl_fn();

    pub struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        pub fn new(width: u32, height: u32) -> Self {
            Rectangle { width, height }
        }
        pub fn width(&self) -> u32 {
            return self.width;
        }
    }


    let rect1 = Rectangle::new(30,30);
    print!("{:?}",rect1.width());
}

fn guess_number (){
    let range_number:u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("please input your guess number:");
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess).expect("fail to read line");
        let guess:u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_err)=>continue
        };
        match guess.cmp(&range_number) {
            Ordering::Less =>println!("too small!"),
            Ordering::Greater =>println!("too big!"),
            Ordering::Equal =>{
                println!("you win!");
                break;
            }
        }
    }
}

fn slice_string(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

fn struct_object(){
    #[derive(Debug)]
    struct User {
        name: String,
        age: u8
    }

    let user = User {
        name: String::from("test"),
        age: 18
    };

    print!("{:#?}",user);
    dbg!(&user);
}

fn impl_fn(){

    #[derive(Debug)]
    struct Rect {
        width: u32,
        height: u32
    }

    impl Rect {
        fn get_width(&self) -> u32 { self.width}

        fn new_rect(size: u32) -> Self {
            Self { 
                width: size,
                height: size
            }
        }
    }

    let react1 = Rect {
        width:100,
        height:100
    };

    let react2 = Rect::new_rect(200);

    print!("{:#?}",react1.get_width());

    dbg!(&react2);
}

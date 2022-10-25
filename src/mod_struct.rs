#[derive(Debug)]
pub struct Rectangle {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Rectangle {
    pub(crate) fn area(&self) -> u32 {
        self.width * self.height
    }
    pub(crate) fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
pub enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
pub fn plus_one(x: Option<i32>) -> Option<i32> {
    // Some(x).map(|i| i+1)
    x.map(|i| i + 1)
}

pub fn test1() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Rect1 area {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    // println!("rect1 is {:#?}", rect1);
    dbg!(&rect1);

    value_in_cents(Coin::Quarter(UsState::Alaska));
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}

// перечисление 
enum Direction {
    Up,
    Down,
    Left,
    Right
}


enum Shape{
    Circul(f64),
    Rectangl {width: f32, height: f32},
}
fn move_player(moving: Direction){
    match moving{
        Direction::Up => println!("Moving up!"),
        Direction::Down => println!("Moving down("),
        Direction::Left => println!("Go to left)"),
        Direction::Right => println!("Go to right $"),
    }
}

fn main() {
    let moving = Direction::Up;
    move_player(moving);

    let shape = Shape::Rectangl{width: 4.0, height: 1.0};

    match shape{
        Shape::Circul(radius) => println!("The circul with radius {radius}"),
        Shape::Rectangl{width, height} => println!("The rectangel with parametrs {} and {}",width,height),
    }
}
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

struct Color(u8, u8, u8);

// единична структура
struct Marker;

struct Rectangel {
    width: f64,
    height: f64,
}

impl Rectangel{
    fn area(&self) -> f64 {
        self.width * self.height 
    }

    fn perimeter(&self) -> f64 {
        (self.width + self.height) * 2.0 
    }
}
fn main(){
    let point = Point{x: 3.0, y: 4.5, z: 6.34};
    println!("Point coord: {}, {}, {}", point.x, point.y,point.z);

    let red = Color(255,0,0);
    println!("Color type: {}, {}, {}", red.0, red.1, red.2);

    let _marker = Marker;

    let rect1 = Rectangel{width: 3.45, height: 12.4};
    println!("Area {}",rect1.area());
    println!("Perimeter {}",rect1.perimeter());

}
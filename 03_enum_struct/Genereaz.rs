// обобщенный тип 
struct Pair <T> {
    first: T,
    second: T,
}
fn main() {
    let pair = Pair{
        first: 10, second: 45
    };
    println!("{}, {}", pair.first, pair.second);
}
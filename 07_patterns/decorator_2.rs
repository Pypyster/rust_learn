mod decorator;
pub use crate::decorator::*;

fn main(){
    let with_tax = TaxDecorator::new(Box::new(BasePrice));
    let price = 10.21;
    let after_tax = with_tax.calculate(price);
    
    let with_dec = DiscountDecorator::new(Box::new(with_tax));
    println!("{:#?}", with_dec);

    let finishPrice = with_dec.calculate(price);

    println!("Start price: {},after tax:{}, finished: {}", price,after_tax, finishPrice);
}
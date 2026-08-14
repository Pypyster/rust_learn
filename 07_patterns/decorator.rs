use std::fmt::Debug;

pub trait Price: Debug {
    fn calculate(&self, price: f32) -> f32;
}

#[derive(Debug)]
pub struct BasePrice;

impl Price for BasePrice {
    fn calculate(&self, price: f32) -> f32 {
        price
    }
}

#[derive(Debug)]
pub struct TaxDecorator {
    decorated: Box<dyn Price>,
}

impl TaxDecorator {
    pub fn new(decorated: Box<dyn Price>) -> Self {
        Self { decorated }
    }
}

impl Price for TaxDecorator {
    fn calculate(&self, price: f32) -> f32 {
        self.decorated.calculate(price) * 1.5
    }
}

#[derive(Debug)]
pub struct DiscountDecorator {
    decorated: Box<dyn Price>,
}

impl DiscountDecorator {
    pub fn new(decorated: Box<dyn Price>) -> Self {
        Self { decorated }
    }
}

impl Price for DiscountDecorator {
    fn calculate(&self, price: f32) -> f32 {
        self.decorated.calculate(price) * 0.75
    }
}
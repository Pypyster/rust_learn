mod visitor_1;
pub use crate::visitor_1::*;

fn main() {
    let mut float_metric = get_float_metric();
    let mut int_metric = get_int_metric();
    println!("Int: {}, float: {}", int_metric.get_report(),float_metric.get_report());

    int_metric.accept(&IncreaseeVisitor);
    float_metric.accept(&DecreaseVisitor);
    println!("Int: {}, float: {}", int_metric.get_report(),float_metric.get_report());

    int_metric.accept(&TwiceVisitor);
    float_metric.accept(&TwiceVisitor);
    println!("Int: {}, float: {}", int_metric.get_report(),float_metric.get_report());

}

fn get_float_metric() -> impl Metric{
    FloatMetric::default()
}

fn get_int_metric() -> impl Metric{
    IntMetric::default()
}

struct TwiceVisitor;

impl Visitor for TwiceVisitor  {
    fn visit_float(&self, metric: &mut FloatMetric) {
        metric.0 /= 2.5;
    }
    fn visit_int(&self, metric:&mut IntMetric) {
        metric.0 *= 4;
    }
}
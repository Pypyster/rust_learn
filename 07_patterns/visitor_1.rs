#[derive(Default)]
pub struct FloatMetric(pub f32);

#[derive(Default)]
pub struct IntMetric(pub i32);

pub trait Metric {
    fn get_report(&self)->String;
    fn accept(&mut self, v: &impl Visitor);
}
impl Metric for FloatMetric  {
    fn get_report(&self)->String {
        self.0.to_string()
    }
    fn accept(&mut self, v: &impl Visitor) {
        v.visit_float(self)
    }
}

impl Metric for IntMetric  {
    fn get_report(&self)->String {
        self.0.to_string()
    }
    fn accept(&mut self, v: &impl Visitor) {
        v.visit_int(self)
    }
}

pub trait Visitor {
    fn visit_float(&self, metric: &mut FloatMetric);
    fn visit_int(&self, metric:&mut IntMetric);
}

pub struct IncreaseeVisitor;

impl Visitor for IncreaseeVisitor {
    fn visit_float(&self, metric: &mut FloatMetric) {
        metric.0 += 5.04;
    }
    fn visit_int(&self, metric:&mut IntMetric) {
        metric.0 += 7;
    }
}

pub struct DecreaseVisitor;
impl Visitor for DecreaseVisitor {
    fn visit_float(&self, metric: &mut FloatMetric) {
        metric.0 -= 4.17;
    }
    fn visit_int(&self, metric:&mut IntMetric) {
        metric.0 -= 11;
    }
}
mod factory;
pub use crate::factory::*;
fn main() {
    let json_factory = json_factory();
    let reporter_j = json_factory.create_reporter();
    let analizer_j 
        = json_factory.create_analizer();
    println!("Json analize result: {}", analizer_j.analize(reporter_j.report()));

    let xml_factory = xml_factory();
    let reporter = xml_factory.create_reporter();
    let analizer = xml_factory.create_analizer();
    println!("Xml analize result: {}", analizer.analize(reporter.report()));

    creat_factory(xml_factory);
}

fn creat_factory<F:  SalesFactory>(factory: F){
    let reporter = factory.create_reporter();
    let analizer = factory.create_analizer();
    println!("Analize result: {}", analizer.analize(reporter.report()));
}
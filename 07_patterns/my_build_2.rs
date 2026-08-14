mod my_build;
pub use crate::my_build::*;

fn main() -> Result<(), ConfigError> {
    let mut pml = PML::new();

    pml.pml_layers(16)
        .source(true);

    let mut builder = ConfigBuilder::new();

    builder
        .with_dt(0.5e-12)
        .with_threads(4)
        .with_save_every(50)
        .with_boundary(pml.clone());

    let config = builder.build()?;

    println!("{config:#?}");

    /*let mut builder_1 = SimulationConfig::builder();
    builder_1.with_boundary(pml).with_dt(0.00065);
    let config_1 = builder_1.build()?;
    println!("{config_1:#?}");*/

    let mut builder3 = SimulationConfig::builder();
    builder3
    .with_grid(500, 300)
    .with_dx(1e-3)
    .with_time_steps(20_000)
    .with_dt(0.5e-12)
    .with_threads(8)
    .with_boundary(pml);
    let config3 = builder3.build()?;
    println!("{config3:#?}");

    Ok(())
}
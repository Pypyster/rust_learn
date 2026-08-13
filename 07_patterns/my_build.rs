#[derive(Default,Debug,Clone, Copy)]
pub struct PML {
    pml_layers: u8,
    source: bool,
}

impl PML{
    pub fn new() -> Self{
        Self::default()
    }
    pub fn pml_layers(&mut self, layers: u8) -> &mut Self{
        self.pml_layers = layers;
        self 
    }

    pub fn source(&mut self, source: bool) -> &mut Self{
        self.source = source;
        self
    }
}
#[derive(Debug)]
pub enum ConfigError {
    InvalidTimeStep,
    InvalidThreadCount,
}

#[derive(Default, Debug)]
pub struct SimulationConfig {
    grid_x: u16,
    grid_y: u16,
    dx: f64,
    time_steps: u128,

    dt: f64,
    boundary: PML,
    save_every: u32,
    threads: u8,
}

pub struct ConfigBuilder {
    config: SimulationConfig,
}

impl ConfigBuilder {
    pub fn with_grid(&mut self, grid_x: u16, grid_y: u16) -> &mut Self{
        self.config.grid_x = grid_x;
        self.config.grid_y = grid_y;
        self
    }
    pub fn with_dx(&mut self, dx: f64) -> &mut Self {
        self.config.dx = dx;
        self
    }

    pub fn with_dt(&mut self, dt: f64) -> &mut Self {
        self.config.dt = dt;
        self
    }

    pub fn with_boundary(&mut self, boundary: PML) -> &mut Self {
        self.config.boundary = boundary;
        self
    }

    pub fn with_threads(&mut self, threads: u8) -> &mut Self {
        self.config.threads = threads;
        self
    }

    pub fn with_save_every(&mut self, save_every: u32) -> &mut Self {
        self.config.save_every = save_every;
        self
    }

    pub fn with_time_steps(&mut self, time_steps: u128) -> &mut Self{
        self.config.time_steps = time_steps;
        self
    }
}

pub trait Builder: Sized {
    type Target;
    type Error;

    fn new() -> Self;
    fn build(self) -> Result<Self::Target, Self::Error>;
}

impl Builder for ConfigBuilder {
    type Target = SimulationConfig;
    type Error = ConfigError;
    fn new() -> Self {
        Self { config: SimulationConfig::default(), }
    }
    fn build(self) -> Result<Self::Target, Self::Error> {
        if self.config.dt <= 0.0 {
            return Err(ConfigError::InvalidTimeStep);
        }

        if self.config.threads == 0 {
            return Err(ConfigError::InvalidThreadCount) ;
        }

        Ok(self.config)
    }
} 

pub trait Create: Sized {
    type Builder: Builder<Target = Self>;

    fn builder() -> Self::Builder;
}
impl Create for SimulationConfig {
    type Builder = ConfigBuilder;

    fn builder() -> Self::Builder {
        ConfigBuilder::new()
    }
}
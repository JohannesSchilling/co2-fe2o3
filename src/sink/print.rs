use async_trait::async_trait;

use super::Sink;


#[derive(Deserialize)]
pub struct PrintConfig {}

pub struct PrintSink {
    points: Vec<super::Measurement>,
}

#[async_trait]
impl Sink for PrintSink {
    fn add_measurement(&mut self, measurement: &super::Measurement) {
        self.points.push(measurement.to_owned());
    }

    async fn submit(&mut self) {
        self.points.retain(|point| {
            println!("{}", point);
            false
        });
    }
}

impl PrintSink {
    pub fn from_config(_config: &PrintConfig) -> Box<dyn Sink> {
        Box::new(PrintSink { points: Vec::new() })
    }
}

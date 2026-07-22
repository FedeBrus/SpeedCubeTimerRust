use std::time::Instant;

pub struct Timer {
    start: Option<Instant>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: None,
        }
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }

    pub fn stop(&mut self) -> Option<u128> {
        self.start.take()
            .map(|start| start.elapsed().as_millis())
    }
}

use std::io::Write;
use std::net::TcpStream;
use std::time::{Duration, Instant};

use crate::color::Color;
use crate::project::Project;

pub struct Runner {
    pub project: Project,
    pub stream: TcpStream,
    pub leds: Vec<Color>,
    pub frame: usize,
    pub start_time: Instant,
}

impl Runner {
    pub fn new(project: Project, address: &str) -> Self {
        let stream = TcpStream::connect(address).unwrap();
        let leds = vec![Color::black(); 64];

        Self {
            project,
            stream,
            leds,
            frame: 0,
            start_time: Instant::now(),
        }
    }

    pub fn proceed(&mut self) {
        self.project.clone().hydrate(self);

        self.send_to_zeevonk();
        self.frame += 1;

        std::thread::sleep(Duration::from_micros(
            1_000_000 / self.project.framerate as u64,
        ));
    }

    pub fn update_project(&mut self, project: Project) {
        self.project = project;
    }

    fn send_to_zeevonk(&mut self) {
        let led_count_bytes = u16::to_be_bytes(64);
        let mut color_bytes = Vec::new();
        for color in self.leds.iter() {
            color_bytes.append(&mut color.as_bytes().to_vec());
        }
        let mut bytes = led_count_bytes.to_vec();
        bytes.append(&mut color_bytes);

        self.stream.write_all(bytes.as_slice()).unwrap();
    }
}

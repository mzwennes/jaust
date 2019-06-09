use harsh::{Harsh, HarshBuilder};
use rand::Rng;

pub trait Shortener {
    fn next_id(&mut self) -> String;
}

pub struct UrlShortener {
    id: u64,
    generator: Harsh,
}

impl UrlShortener {
    pub fn new() -> UrlShortener {
        let mut rng = rand::thread_rng();
        let salt: Vec<u8> = (0..10).map(|_| rng.gen()).collect();

        let harsh = HarshBuilder::new().salt(salt).length(5)
            .init().unwrap();

        UrlShortener {
            id: 0,
            generator: harsh,
        }
    }
}

impl Shortener for UrlShortener {
    fn next_id(&mut self) -> String {
        let hash = self.generator.encode(&[self.id]).unwrap();
        self.id += 1;
        hash
    }
}
use harsh::{Harsh, HarshBuilder};

pub trait Shortener {
    fn next_id(&mut self) -> String;
}

pub struct UrlShortener {
    id: u64,
    generator: Harsh,
}

impl UrlShortener {
    pub fn new() -> UrlShortener {
        let harsh = HarshBuilder::new().length(5).init().unwrap();
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
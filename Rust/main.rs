use std::time::Instant;

pub mod Tools;
pub mod Pages;
use Pages::Page1::page1;

struct Page {
}

impl Page {
    fn new() -> Page
    {
        Page {}
    }
}

impl Pages::Page1::page1 for Page {}

fn main()
{
    let now = Instant::now();

    let pg: Page = Page::new();

    println!("Problem 1: {}", pg.mult3_5());
    println!("Problem 2: {}", pg.evenFib());
    println!("Problem 3: {}", pg.largestPrime());
    println!("Problem 4: {}", pg.pal());
    println!("Problem 5: {}", pg.smallestMult());
    println!("Problem 6: {}", pg.diffsquares());
    println!("Problem 7: {}", pg.numPrime());
    println!("Problem 8: {}", pg.productInSeries());
    println!("Problem 9: {}", pg.pythTriplet());
    println!("Problem 10: {}", pg.sumPrime());

    let time: String = now.elapsed().as_secs().to_string() + "." + &(now.elapsed().subsec_nanos() as f64 / (1 as f64).powi(10)).to_string();
    println!("Running time - {} seconds", time);
}

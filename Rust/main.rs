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

    println!("Problem 1: {}", pg.mult3_5()); // problem 1 - 233168
    println!("Problem 2: {}", pg.evenFib()); // problem 2 - 4613732
    println!("Problem 3: {}", pg.largestPrime()); // problem 3 - 6857
    println!("Problem 4: {}", pg.pal()); // problem 4 - 906609
    println!("Problem 5: {}", pg.smallestMult()); // problem 5 - 232792560
    println!("Problem 6: {}", pg.diffsquares()); // problem 6 - 25164150
    println!("Problem 7: {}", pg.numPrime()); // problem 7 - 104743
    println!("Problem 8: {}", pg.productInSeries()); // problem 8 - 23514624000
    println!("Problem 9: {}", pg.pythTriplet()); // problem 9 - 31875000
    println!("Problem 10: {}", pg.sumPrime()); // problem 10 - 142913828922

    let time: String = now.elapsed().as_secs().to_string() + "." + &(now.elapsed().subsec_nanos() as f64 / (1 as f64).powi(10)).to_string();
    println!("Running time - {} seconds", time);
}

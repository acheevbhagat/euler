#![allow(non_snake_case)]

use Tools;

pub trait page1 {

    fn mult3_5(&self) -> u32
    {
        let mut res: u32 = 0;
        for i in 1..1000 {
            if i % 3 == 0 || i % 5 == 0 {
                res += i;
            }
        }
        res
    }

    fn evenFib(&self) -> u32
    {
        let mut i = 2;
        let mut prev = 1;
        let mut sum: u32 = 0;
        while i < 4000000 {
            if i % 2 == 0 {
                sum += i;
            }
            let temp = i + prev;
            prev = i;
            i = temp;
        }
        sum
    }

    fn largestPrime(&self) -> u64
    {
        let mut largestprime: u64 = 600851475143;
        let mut currentdivisor: u64 = 2;
        while largestprime > currentdivisor {
            if largestprime % currentdivisor == 0 {
                largestprime = largestprime / currentdivisor;
                currentdivisor = 2;
            } else {
                currentdivisor += 1;
            }
        }
        currentdivisor
    }

    fn pal(&self) -> u32
    {
        let mut res = 0;
        for x in 100..999 {
            for y in x..999 {
                if Tools::palindrome(x * y) && (x * y > res) {
                    res = x * y;
                }
            }
        }
        res
    }

    fn smallestMult(&self) -> u64
    {
        let mut res: Vec<u64> = Vec::new();
        for i in 1..21 {
            let mut temp = Tools::primeFactors(i);
            for z in &res {
                if temp.contains(&z) {
                    let mut counter: usize = 0;
                    for j in &temp {
                        if j == z {
                            break;
                        } else {
                            counter += 1;
                        }
                    }
                    temp.remove(counter);
                }
            }
            res.extend(&temp);
        }
        let mut val: u64 = 1;
        for x in res {
            val = val * x;
        }
        val
    }

    fn diffsquares(&self) -> u64
    {
        let mut sumsquare: i64 = 0;
        let mut squaresum: i64 = 0;
        for i in 0..101 {
            sumsquare += i * i;
            squaresum += i;
        }
        squaresum = squaresum * squaresum;
        Tools::abs(squaresum - sumsquare)
    }

    fn numPrime(&self) -> u64
    {
        let mut count: u64 = 0;
        let mut start: u64 = 2;
        while count != 10001 {
            if Tools::prime(start) {
                count += 1
            }
            start += 1;
        }
        start - 1
    }

    fn productInSeries(&self) -> u64
    {
        let input: String = String::from("7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450");
        let mut maxVal = 0;
        let input_bytes = input.as_bytes();
        let mut largest_string: &[u8] = &input_bytes[0..1];

        let span_width = 13;

        for i in 0..(input_bytes.len() - span_width + 1) {
            let mut sum = 1u64;
            for j in 0..(span_width) {
                sum *= (input_bytes[i + j] - 48) as u64;
            }
            if sum > maxVal {
                maxVal = sum;
                largest_string = &input_bytes[i..(i+span_width)];
            }
        }
        maxVal
    }

    fn pythTriplet(&self) -> u32
    {
        let mut a: u32 = 0;
        let mut b: u32 = 0;
        let mut c: u32 = 0;
        for i in 0..1000 {
            for j in (i + 1)..1000 {
                let m: u32 = i as u32;
                let n: u32 = j as u32;
                if (i + j) as f64 == (1000.0 - Tools::sqrt(((m.pow(2) + n.pow(2)) as f64))) {
                    a = m;
                    b = n;
                    c = 1000 - (a + b);
                }
            }
        }
        a * b * c
    }

    fn sumPrime(&self) -> u64
    {
        let primes = Tools::sieve(2000000);
        primes.iter().sum()
    }
}

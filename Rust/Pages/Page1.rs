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
	
	fn gridProd(&self) -> u64
	{
		let mut L: Vec<String> = Vec::new();
		L.push(String::from("08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08"));
		L.push(String::from("49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00"));
        L.push(String::from("81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65"));
        L.push(String::from("52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91"));
        L.push(String::from("22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80"));
        L.push(String::from("24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50"));
        L.push(String::from("32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70"));
        L.push(String::from("67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21"));
        L.push(String::from("24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72"));
        L.push(String::from("21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95"));
        L.push(String::from("78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92"));
        L.push(String::from("16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57"));
        L.push(String::from("86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58"));
		L.push(String::from("22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80"));
        L.push(String::from("24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50"));
        L.push(String::from("32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70"));
        L.push(String::from("67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21"));
        L.push(String::from("24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72"));
        L.push(String::from("21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95"));
        L.push(String::from("78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92"));
        L.push(String::from("16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57"));
        L.push(String::from("86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58"));
        L.push(String::from("19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40"));
        L.push(String::from("04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66"));
        L.push(String::from("88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69"));
        L.push(String::from("04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36"));
        L.push(String::from("20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16"));
        L.push(String::from("20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54"));
        L.push(String::from("01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48"));
		let mut m = Vec::new();
		for s in L {
			m.push(s.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect());
		}
		drop(L);
		let mut res: u64 = 0;
		for i in 0..20 {
			for j in 0..16 {
			}
		}
	}
}

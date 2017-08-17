#![allow(non_snake_case)]
#![warn(unused_assignments)]

use std::iter;

pub fn prime(n: u64) -> bool
{
    let mut res: bool = true;
    if n <= 1 {
        res = false;
    } else if n <= 3 {
        res = true;
    } else if n % 2 == 0 {
        res = false;
    }
    let root: u64 = (sqrt(n as f64) as u64) + 1;
    for mut i in 3..root {
        if n % i == 0 {
            res = false;
        }
        i += 1;
    }
    res
}

pub fn sqrt(val: f64) -> f64
{
    sqrtHelper(val, 1.0)
}

fn sqrtHelper(val: f64, approx: f64) -> f64
{
    let betterapprox: f64 = 0.5 * (approx + val / approx);
    if betterapprox == approx {
        return approx;
    } else {
        return sqrtHelper(val, betterapprox);
    }
}

pub fn root(n: f64, power: i32) -> f64
{
    n.powi(1 / power)
}

pub fn palindrome(n: u32) -> bool
{
    let arrN = n.to_string();
    let mut res: bool = true;
    for (i1, i2) in arrN.chars().zip(arrN.chars().rev()) {
        if i1 != i2 {
            res = false;
        }
    }
    res
}

pub fn factorial(n: &u64) -> u64
{
    if n > &1 {
        return n * factorial(&(n - 1));
    } else {
        return 1;
    }
}

pub fn factors(n: u64) -> Vec<u64>
{
    let mut res: Vec<u64> = Vec::new();
    let mut d: u64 = 1;
    while d <= sqrt(n as f64) as u64 + 1 {
        if n % d == 0 {
            res.push(d);
            res.push(n / d);
        }
        d += 1;
    }
    res.sort();
    res
}

pub fn primeFactors(n: u64) -> Vec<u64>
{
    let mut number = n;
    let mut prime_numbers: Vec<u64> = Vec::new();
    let mut candidate = 2;

    while number > 1 {
        while number % candidate == 0 {
            prime_numbers.push(candidate);
            number /= candidate;
        }
        candidate += 1;
    }
    prime_numbers.sort();
    prime_numbers
}

pub fn abs(n: i64) -> u64
{
    if n > 0 {
        return n as u64;
    } else {
        return (-1 * n) as u64;
    }
}

pub fn pyth(a: u32, b: u32, c: u32) -> bool
{
    a.pow(2) + b.pow(2) == c.pow(2)
}

pub fn sieve(n: u64) -> Vec<u64>
{
    let mut arr: Vec<bool> = Vec::new();
    arr.extend(iter::repeat(true).take((n) as usize));
    let mut res: Vec<u64> = Vec::new();
    let primeLimit: u64 = sqrt(n as f64) as u64;
    for mut i in 2..(n) {
        if arr[i as usize] {
            res.push(i);
            if i < primeLimit {
                let mut j: u64 = i * i;
                while j < n {
                    arr[j as usize] = false;
                    j += i;
                }
            }
        }
        i += 1;
    }
    res
}

pub struct Node<T>(_varargs: &[T]) {
	data: Vec<T> = _varargs.to_vec(),
}

impl <T> Node<T>

fn main() {
    // test_generating_primes();
    test_cracking_key();

    // let primes = first_n_numbers(range.1);
    // println!("Generated primes: {:?}", primes.len());
}

fn test_generating_primes() {
    let (range, primes) = ((100000000, 200000000), 200);

    let mut filter = PrimeFilter::new(range.0, primes);
    println!("Start filtering, working on {:?} primes", primes);
    let generated = filter.filter(range.1);
    println!("Primes are filtered, found: {:?}", generated.len());
    println!("Numbers range: <{:?}, {:?}>", range.0, range.1);
    println!("Total nummbers to compute: {:?}", range.1 - range.0);

    check_percentage_of_primes(generated.clone());
}

fn test_cracking_key() {
    // let key: u64 = 11746811 * 1993209007;
    let key: u64 = 1993209007 * 1993209007;
    println!("Trying to crack the key: {:?}", key);
    let cracked = crack(key, 100);
    // let cracked = crack_classic(key);
    println!("Cracked, primes: {:?} ", cracked);
}

fn check_percentage_of_primes(prime: Vec<u64>) {
    let (mut quantity, mut i) = (0,0);
    let all = prime.len();
    println!("Counting percentage of primes. Hold on!");

    for p in prime {
        if check_prime(p) {
            quantity += 1;
            // println!("Found a prime! It is a {:?}, hurray!, Left to check: {:?}", p, all - i);
        }
        i += 1;
    }

    let percentage = (quantity as f64 / all as f64) * 100.0;

    println!("Found: {:?} prime numbers", quantity);
    println!("Percentage in a sample: {:?}%", percentage);
}

fn first_n_primes(quantity: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let (mut k, mut n) = (0, 2);

    while quantity > k {
        if check_prime(n) { primes.push(n); k += 1; }
        n += 1;
    }

    primes
}

fn first_n_numbers(quantity: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut n = 2;

    while quantity > n {
        if check_prime(n) { primes.push(n);}
        n += 1;
    }

    primes
}

fn check_prime(n: u64) -> bool {
    if n == 2 { return true; }

    match n > 2 && n % 2 != 0 {
        false => false,
        true  => {
            let stop = (n as f64).sqrt().floor() as u64;
            let mut modulo = 3;

            while n % modulo != 0 && modulo < stop { modulo += 2; }
            modulo >= stop
        }
    }
}

fn get_max_prime(n: u64) -> u64 {
    if n == 2 { return n; }

    match n > 2 && n % 2 != 0 {
        false => n,
        true  => {
            let stop = (n as f64).sqrt().floor() as u64;
            let mut modulo = 3;

            while n % modulo != 0 && modulo < stop { modulo += 2; }
            modulo
        }
    }
}

fn crack(key: u64, quality: u64) -> (u64, u64) {
    let m = (key as f64).sqrt().floor() as u64;
    let mut n = (m as f64).sqrt().floor() as u64;
    let batch = 10000;

    println!("Range of search: <{:?}, {:?}>, total numbers: {:?}", n, m, m-n);
    let mut filter = PrimeFilter::new(n, quality);

    while n < m {
        n += batch;
        let primes = filter.filter(n);

        for prime in primes {
            match key % prime == 0 {
                true => return (prime, key/prime),
                false => ()
            }
        }
    }
    (0,0)
}

fn crack_classic(key: u64) -> (u64, u64) {
    let m = (key as f64).sqrt().floor() as u64;
    let n = (m as f64).sqrt().floor() as u64;

    println!("Start filtering, cracking key: {:?}, sqrt^2: {:?}, sqrt^4: {:?}", key, m, n);

    let prime = get_max_prime(key);
    (prime, key/prime)
}


#[derive(Clone, Debug)]
pub struct PrimeFilter {
    first: u64,
    rotors: Vec<(u64, u64)>
}

impl  PrimeFilter {
    pub fn new(first: u64, primes_quantity: u64) -> Self {
        let mut primes = first_n_primes(primes_quantity);

        let rotors = primes.iter()
            .map(|p| (*p, first % *p))
            .collect::<Vec<(_,_)>>();

        PrimeFilter { first, rotors }
    }

    pub fn filter(&mut self, last: u64) -> Vec<u64> {
        let mut primes = Vec::new();
        let mut actual = self.first;

        if actual % 2 == 0 {
            self.rotate(1);
            actual += 1;
        }

        while actual < last {
            if self.rotate(2) {
                primes.push(actual);
            }
            actual += 2;
        }

        self.first = actual;

        primes
    }

    fn rotate(&mut self, steps: u64) -> bool {
        let mut np = true;
        let rotors = self.rotors
            .iter_mut()
            .map(|p| {
                if np && p.1 == 0 { np = false; }
                Self::turn(&p, steps)
            })
            .collect::<Vec<(_,_)>>();

        self.rotors = rotors;

        np
    }

    // fn rotate(&mut self, steps: u64) -> bool {
    //     let mut np = true;

    //     for mut rotor in self.rotors {
    //         if np && rotor.1 == 0 { np = false; }
    //         *rotor = Self::turn(rotor, steps)
    //     }

    //     np
    // }

    fn turn(rotor: &(u64, u64), steps: u64) -> (u64, u64) {
        match rotor.0 > rotor.1 + steps {
            true => (rotor.0, rotor.1 + steps),
            false => (rotor.0, rotor.1 + steps - rotor.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod prime_number_check {
        use super::*;
        #[test]
        fn mixed_examples() {
            assert!(check_prime(23));
            assert!(check_prime(29));
            assert!(check_prime(11746811));
            assert!(check_prime(1993209007));
            assert!(!check_prime(1993209005));
            assert!(!check_prime(11746813));
            assert!(!check_prime(24));
            assert!(!check_prime(69));
            assert!(!check_prime(144));
        }
    }
}

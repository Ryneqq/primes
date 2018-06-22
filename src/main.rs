fn main() {
    let (range, primes) = ((1000000000000000, 1000000000000100), 1000);
    // let generated = get_natural_numbers(range.0, range.1);

    // let range = (200000, 1100000);
    // let mut filter = PrimeFilter::new(range.0, range.1, primes);
    // println!("Start filtering, working on {:?} primes", primes);
    // let generated = filter.filter();
    // // println!("Sample: {:?}", generated);
    // println!("Primes are filtered, found: {:?}", generated.len());
    // println!("Numbers range: <{:?}, {:?}>", range.0, range.1);
    // println!("Total nummbers to compute: {:?}", range.1 - range.0);

    // check_percentage_of_primes(generated.clone());
    // 1000000000000037
153015847,
    let key = 11746811 * 1993209007;
    let cracked = crack(key, 3);
    println!("We cracked the key: {:?}, primes: {:?} ", key, cracked);
}

fn check_percentage_of_primes(prime: Vec<u64>) {
    let (mut quantity, mut i) = (0,0);
    let all = prime.len();
    println!("Counting percentage of primes. Hold on!");

    for p in prime {
        if check_prime(p) {
            quantity += 1;
            println!("Found a prime! It is a {:?}, hurray!, Left to check: {:?}", p, all - i);
        }
        i += 1;
    }

    let percentage = (quantity as f64 / all as f64) * 100.0;

    println!("Found: {:?} prime numbers", quantity);
    println!("Percentage in a sample: {:?}%", percentage);
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

fn crack(key: u64, quality: u64) -> (u64, u64) {
    let m = (key as f64).sqrt().floor() as u64;
    let n = (m as f64).sqrt().floor() as u64;
    
    println!("Start filtering, cracking key: {:?}, sqrt^2: {:?}, sqrt^4: {:?}", key, m, n);
    let mut filter = PrimeFilter::new(n, m, quality);
    let primes = filter.filter();
    println!("Found {:?} primes", primes.len());

    for prime in primes {
         match key % prime == 0 {
            true => return (prime, key/prime),
            false => ()
        }
    }
    (0,0)
}

#[derive(Clone, Debug)]
pub struct PrimeFilter {
    first: u64,
    last:  u64,
    rotors: Vec<(u64, u64)>
}

impl  PrimeFilter {
    pub fn new(first: u64, last: u64, primes_quantity: u64) -> Self {
        let mut primes = Vec::new();
        let (mut k, mut n) = (0, 2);

        while primes_quantity > k {
            if check_prime(n) { primes.push(n); k += 1; }
            n += 1;
        }

        let rotors = primes.iter()
            .map(|p| (*p, first % *p))
            .collect::<Vec<(_,_)>>();

        PrimeFilter { first, last, rotors }
    }

    pub fn filter(&mut self) -> Vec<u64> {
        let mut primes = Vec::new();
        let mut actual = self.first;

        if actual % 2 == 0 {
            if self.check_prime(){
                primes.push(actual);
            }
            self.rotate(1);
            actual += 1;
        }

        while actual < self.last {
            if self.check_prime() {
                primes.push(actual);
            }
            self.rotate(2);
            actual += 2;
        }

        primes
    }

    fn check_prime(&mut self) -> bool {
        for rotor in self.rotors.iter() {
            if rotor.1 == 0 { return false; }
        }

        true
    }

    fn rotate(&mut self, steps: u64) {
        let rotors = self.rotors
            .iter_mut()
            .map(|p| Self::turn(&p, steps))
            .collect::<Vec<(_,_)>>();

        self.rotors = rotors;
    }

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

        #[test]
        fn thousand_primes() {
            let prime = prime(0);
            prime.iter()
                .map(|p| assert!(check_prime(*p), format!("not a prime: {:?}", *p)))
                .next();
        }
    }
}

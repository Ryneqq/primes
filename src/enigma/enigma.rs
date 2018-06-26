use enigma::rotor::Rotor;


pub struct Enigma {
    rotors: Vec<Rotor>,
    range: (u64, u64)
}

impl Enigma {
    pub fn new(quality: u32, range: (u64, u64)) -> Self {
        let primes = prime_filter(quality);

        let rotors = primes.iter()
            .map(|p| Rotor::new(*p, (range.0 % *p as u64) as u32))
            .collect();

        Enigma { rotors, range }
    }

    pub fn crack() {}
}

fn prime_filter(quantity: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    let (mut k, mut n) = (0, 2);

    while quantity > k {
        if check_prime(n) { primes.push(n); k += 1; }
        n += 1;
    }

    primes
}

fn check_prime(n: u32) -> bool {
    if n == 2 { return true; }

    match n > 2 && n % 2 != 0 {
        false => false,
        true  => {
            let stop = (n as f32).sqrt() as u32 + 1;
            let mut modulo = 3;

            while n % modulo != 0 && modulo < stop { modulo += 2; }
            modulo >= stop
        }
    }
}

    // pub fn filter(&mut self, last: u32) -> Vec<u32> {
    //     let mut primes = Vec::new();
    //     let mut actual = self.first;

    //     if actual % 2 == 0 {
    //         self.rotate(1);
    //         actual += 1;
    //     }

    //     while actual < last {
    //         if self.rotate(2) {
    //             primes.push(actual);
    //         }
    //         actual += 2;
    //     }

    //     self.first = actual;

    //     primes
    // }

    // fn rotate(&mut self, steps: u32) -> bool {
    //     let mut np = true;
    //     let rotors = self.rotors
    //         .iter_mut()
    //         .map(|p| {
    //             if np && p.1 == 0 { np = false; }
    //             Self::turn(&p, steps)
    //         })
    //         .collect::<Vec<(_,_)>>();

    //     self.rotors = rotors;

    //     np
    // }

// fn prime_filter(n: u32) {
//     let prime = 2;
//     let mut natural: Vec<u32> = (3..n).collect();

//     natural.iter()
//         .filter(|n| )
// }

// #[cfg(test)]
// mod tests {

//     #[test]
//     fn test() {

//     }
// }
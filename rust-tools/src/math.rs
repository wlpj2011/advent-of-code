pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}


pub fn lcm(a: u64, b: u64) -> u64 {
        if a == 0 || b == 0 {
            return 0;
        }
        (a / gcd(a, b)) * b 
    }

#[cfg(test)]
mod test {
    use super::{gcd, lcm};

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(3,2), 1);
    }

     #[test]
    fn test_lcm() {
        assert_eq!(lcm(3,2), 6);
    }
}
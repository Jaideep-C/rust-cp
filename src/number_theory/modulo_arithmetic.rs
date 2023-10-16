struct ModuloArithmetic {
    modulo: i64,
}

impl ModuloArithmetic {
    fn new(modulo: i64) -> Self {
        Self { modulo }
    }
    fn add(&self, a: i64, b: i64) -> i64 {
        return ((a % self.modulo) + (b % self.modulo)) % self.modulo;
    }
    fn sub(&self, a: i64, b: i64) -> i64 {
        return ((a % self.modulo) - (b % self.modulo) + self.modulo) % self.modulo;
    }
    fn mul(&self, a: i64, b: i64) -> i64 {
        return ((a % self.modulo) * (b % self.modulo)) % self.modulo;
    }
    fn div(&self, a: i64, b: i64) -> i64 {
        return self.mul(a, self.bin_pow(b, self.modulo - 2));
    }
    fn bin_pow(&self, a: i64, b: i64) -> i64 {
        if b == 0 {
            return 1;
        }
        let half = self.bin_pow(a, b / 2);
        return if b % 2 == 0 {
            self.mul(half, half)
        } else {
            self.mul(self.mul(half, half), a)
        };
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const MODULO: i64 = 1_000_000_007;

    #[test]
    fn test_modulo_arithmetic() {
        let modulo_arithmetic = ModuloArithmetic::new(MODULO);
        assert_eq!(modulo_arithmetic.add(1, 2), 3);
        assert_eq!(modulo_arithmetic.sub(2, 2), 0);
        assert_eq!(modulo_arithmetic.mul(2, 2), 4);
        assert_eq!(modulo_arithmetic.bin_pow(2, 2), 4);
        assert_eq!(modulo_arithmetic.div(4, 2), 2);
    }

    #[test]
    fn test_add() {
        let modulo_arithmetic = ModuloArithmetic::new(MODULO);
        for _ in 0..1_000_000_000 {
            let a = rand::random::<i64>() % MODULO;
            let b = rand::random::<i64>() % MODULO;
            let expected = (a + b) % MODULO;
            let actual = modulo_arithmetic.add(a, b);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_sub() {
        let modulo_arithmetic = ModuloArithmetic::new(MODULO);
        for _ in 0..1_000_000_000 {
            let a = rand::random::<i64>() % MODULO;
            let b = rand::random::<i64>() % MODULO;
            let expected = (a - b + MODULO) % MODULO;
            let actual = modulo_arithmetic.sub(a, b);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_mul() {
        let modulo_arithmetic = ModuloArithmetic::new(MODULO);
        for _ in 0..1_000_000_000 {
            let a = rand::random::<i64>() % MODULO;
            let b = rand::random::<i64>() % MODULO;
            let expected = (a * b) % MODULO;
            let actual = modulo_arithmetic.mul(a, b);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_pow() {
        let modulo_arithmetic = ModuloArithmetic::new(MODULO);
        for _ in 0..1_000_000_000 {
            let a = rand::random::<i64>() % MODULO;
            let b = rand::random::<u32>();
            let expected = (a.pow(b)) % MODULO;
            let actual = modulo_arithmetic.bin_pow(a, b as i64);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_div() {
        let modulo_arithmetic = ModuloArithmetic::new(MODULO);
        for _ in 0..1_000_000_000 {
            let a = rand::random::<i64>() % MODULO;
            let b = rand::random::<i64>() % MODULO;
            let expected = (a / b) % MODULO;
            let actual = modulo_arithmetic.div(a, b);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_div_odd() {
        let modulo_arithmetic = ModuloArithmetic::new(MODULO);
        let mut a = 7;
        let mut b = 3;
        let mut expected = (a / b) % MODULO;
        let mut actual = modulo_arithmetic.div(a - (a % b), b);
        assert_eq!(actual, expected);
    }
}
use std::ops::Add;

struct Fraction {
    numerator: u32,
    denominator: u32,
}

impl Fraction {
    fn new(numerator: u32, denominator: u32) -> Self {
        let gcf_value = Self::gcf(numerator, denominator);
        Fraction {
            numerator: numerator / gcf_value,
            denominator: denominator / gcf_value,
        }
    }

    fn gcf(value1: u32, value2: u32) -> u32 {
        // ユークリッドの互除法
        // 1. a / b で余り r を求める
        // 2. r > 0 なら b / r で余り r2 を求める
        // 3. r2 > 0 なら r / r2 で余り r3 を求める
        // 4. rnが0になるまで繰り返す
        // 5. 最後に余りが0になったときの除数が最大公約数

        let (mut a, mut b) = if value1 > value2 {
            (value1, value2)
        } else {
            (value2, value1)
        };

        let mut r = a % b;
        while r > 0 {
            a = b;
            b = r;
            r = a % b;
        }
        return b;
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let lcm = self.denominator / Fraction::gcf(self.denominator, other.denominator)
            * other.denominator;
        let a = self.numerator * (lcm / self.denominator);
        let b = other.numerator * (lcm / other.denominator);
        Fraction::new(a + b, lcm)
    }
}

fn main() {
    let a = Fraction::new(8, 18);
    println!("a = {}/{}", a.numerator, a.denominator);

    let b = Fraction::new(1, 2);
    println!("b = {}/{}", b.numerator, b.denominator);

    let c = a + b;
    println!("a + b = ");
    println!("{}/{}", c.numerator, c.denominator);
}

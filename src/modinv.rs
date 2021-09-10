use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::mem;

/// Represent a i320 with support for carry
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct I320 {
    pub d: [u64; 5]
}

impl I320 {
    pub const fn new(d4: u64, d3: u64, d2: u64, d1: u64, d0: u64) -> Self {
        Self { d: [d0, d1, d2, d3, d4] }
    }

    fn is_even(&self) -> bool {
        self.d[0] & 0x1 == 0x0
    }

    fn is_zero(&self) -> bool {
        (self.d[0] | self.d[1] | self.d[2] | self.d[3] | self.d[4]) == 0
    }

    fn div2(&mut self) {
        let mut t: u64;

        t = self.d[1] & 0x01;
        self.d[0] = (t << 63) | (self.d[0] >> 1);
        t = self.d[2] & 0x01;
        self.d[1] = (t << 63) | (self.d[1] >> 1);
        t = self.d[3] & 0x01;
        self.d[2] = (t << 63) | (self.d[2] >> 1);
        t = self.d[4] & 0x01;
        self.d[3] = (t << 63) | (self.d[3] >> 1);
        t = self.d[4] >> 63;
        self.d[4] = (t << 63) | (self.d[4] >> 1);
    }

    fn div2_mod(&mut self, m: &Self) {
        if !self.is_even() {
            self.add_assign(m);
        }
        self.div2()
    }

    pub fn modinv(&mut self, m: &Self) {
        let mut b = *m;
        let mut x = Self { d: [1, 0, 0, 0, 0] };
        let mut y = Self { d: [0, 0, 0, 0, 0] };

        while !self.is_zero() {
            if self.is_even() {
                self.div2();
                x.div2_mod(m);
            } else {
                if *self < b {
                    mem::swap(self, &mut b);
                    mem::swap(&mut x, &mut y);
                }
                *self -= b;
                self.div2();
                x -= y;
                x.div2_mod(m);
            }
        }
        *self = y;
    }
}

impl fmt::Debug for I320 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:016x}{:016x}{:016x}{:016x}{:016x}",
               self.d[4], self.d[3], self.d[2], self.d[1], self.d[0])
    }
}

impl Add<I320> for I320 {
    type Output = I320;

    fn add(self, rhs: I320) -> I320 {
        let mut r = self;

        r.add_assign(&rhs);
        r
    }
}

impl<'a, 'b> Add<&'a I320> for &'b I320 {
    type Output = I320;

    fn add(self, rhs: &'a I320) -> I320 {
        let mut r = *self;

        r.add_assign(rhs);
        r
    }
}

impl<'a> AddAssign<&'a I320> for I320 {
    fn add_assign(&mut self, rhs: &'a I320) {
        let mut t: u128;

        t = self.d[0] as u128 + rhs.d[0] as u128;
        self.d[0] = t as u64;
        t >>= 64;

        t += self.d[1] as u128 + rhs.d[1] as u128;
        self.d[1] = t as u64;
        t >>= 64;

        t += self.d[2] as u128 + rhs.d[2] as u128;
        self.d[2] = t as u64;
        t >>= 64;

        t += self.d[3] as u128 + rhs.d[3] as u128;
        self.d[3] = t as u64;
        t >>= 64;

        t += self.d[4] as u128 + rhs.d[4] as u128;
        self.d[4] = t as u64;
    }
}

impl AddAssign<I320> for I320 {
    fn add_assign(&mut self, rhs: I320) {
        self.add_assign(&rhs)
    }
}

impl Sub<I320> for I320 {
    type Output = I320;

    fn sub(self, rhs: I320) -> I320 {
        let mut r = self;

        r.sub_assign(&rhs);
        r
    }
}

impl<'a, 'b> Sub<&'a I320> for &'b I320 {
    type Output = I320;

    fn sub(self, rhs: &'a I320) -> I320 {
        let mut r = *self;

        r.sub_assign(rhs);
        r
    }
}

impl<'a> SubAssign<&'a I320> for I320 {
    fn sub_assign(&mut self, rhs: &'a I320) {
        let mut t: u128;

        t = (self.d[0] as u128).wrapping_sub(rhs.d[0] as u128);
        self.d[0] = t as u64;
        t >>= 64;
        t &= 0x01;

        t = (self.d[1] as u128).wrapping_sub(t + rhs.d[1] as u128);
        self.d[1] = t as u64;
        t >>= 64;
        t &= 0x01;

        t = (self.d[2] as u128).wrapping_sub(t + rhs.d[2] as u128);
        self.d[2] = t as u64;
        t >>= 64;
        t &= 0x01;

        t = (self.d[3] as u128).wrapping_sub(t + rhs.d[3] as u128);
        self.d[3] = t as u64;
        t >>= 64;
        t &= 0x01;

        t = (self.d[4] as u128).wrapping_sub(t + rhs.d[4] as u128);
        self.d[4] = t as u64;
    }
}

impl SubAssign<I320> for I320 {
    fn sub_assign(&mut self, rhs: I320) {
        self.sub_assign(&rhs)
    }
}

impl Ord for I320 {
    fn cmp(&self, other: &I320) -> Ordering {
        if self.d[4] > other.d[4] {
            // same sign
            if (self.d[4] ^ other.d[4]) >> 63 == 0 {
                return Ordering::Greater
            } else {
                return Ordering::Less
            }
        }
        if self.d[4] < other.d[4] {
            if (self.d[4] ^ other.d[4]) >> 63 == 0 {
                return Ordering::Less
            } else {
                return Ordering::Greater
            }
        }
        // d[4] == other.d[4], same signs
        for i in (0..4).rev() {
            if self.d[i] > other.d[i] {
                return Ordering::Greater;
            }
            if self.d[i] < other.d[i] {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for I320 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_div2() {
        let mut a = I320::new(0x0000000000000000,
                              0x0000000000000000,
                              0x0000000000000000,
                              0x0000000000000100,
                              0x0000000000000000);
        let b = I320::new(0x0000000000000000,
                          0x0000000000000000,
                          0x0000000000000000,
                          0x0000000000000020,
                          0x0000000000000000);

        a.div2();
        a.div2();
        a.div2();
        assert_eq!(a, b);
    }

    #[test]
    fn it_tests_ordering() {
        let a = I320::new(0x8000000000000000,
                          0x0000000000000000,
                          0x0000000000000000,
                          0x0000000000000000,
                          0x0000000000000000); // -2^319

        let min_1 = I320::new(0xffffffffffffffff,
                          0xffffffffffffffff,
                          0xffffffffffffffff,
                          0xffffffffffffffff,
                          0xffffffffffffffff); // -1

        let b = I320::new(0x7fffffffffffffff,
                          0xffffffffffffffff,
                          0xffffffffffffffff,
                          0xffffffffffffffff,
                          0xffffffffffffffff); // 2^319 - 1
        let n_0 = I320::new(0, 0, 0, 0, 0);
        let n_1 = I320::new(0, 0, 0, 0, 1);

        assert!(a < min_1);
        assert!(min_1 > a);

        assert!(b > a);
        assert!(a < b);

        assert!(b > min_1);
        assert!(min_1 < b);

        assert!(min_1 < n_0);
        assert!(n_0 > min_1);

        assert!(a < n_0);
        assert!(n_0 > a);

        assert!(b > n_0);
        assert!(n_0 < b);

        assert!(n_1 > n_0);
        assert!(n_0 < n_1);

        assert!(b > n_1);
        assert!(n_1 < b);

        assert!(min_1 < n_1);
        assert!(n_1 > min_1);
    }

    #[test]
    fn it_modinv() {
        let mut a = I320::new(0x0000000000000000,
                              0xffffffffffffffff,
                              0xffffffffffffffff,
                              0xffffffffffffffff,
                              0xfffffbfefffffc2f);
        let mut b = I320::new(0x0000000000000000,
                              0x7fffffffffffffff,
                              0xffffffffffffffff,
                              0xffffffffffffffff,
                              0xffffffff7ffffe18);
        let mut c = I320::new(0x0000000000000000,
                              0x0000000000000000,
                              0x0000000000000000,
                              0x0000000000000000,
                              0x0000000000111111);

        let p = I320::new(0x0000000000000000,
                          0xffffffffffffffff,
                          0xffffffffffffffff,
                          0xffffffffffffffff,
                          0xfffffffefffffc2f);

        let res = I320::new(0x0000000000000000,
                            0xb88b76b2b3bfffff,
                            0xffffffffffffffff,
                            0xffffffffffffffff,
                            0xffffffff4774868d);
        let res2 = I320::new(0x0000000000000000,
                             0x0000000000000000,
                             0x0000000000000000,
                             0x0000000000000000,
                             0x0000000000000002);
        let res3 = I320::new(0x0,
                             0x3eb0f23eb0f23eb0,
                             0xf23eb0f23eb0f23e,
                             0xb0f23eb0f23eb0f2,
                             0x3eb0f23e72414b83);

        a.modinv(&p);
        assert_eq!(a, res);

        b.modinv(&p);
        assert_eq!(b, res2);

        c.modinv(&p);
        assert_eq!(c, res3);
    }
}
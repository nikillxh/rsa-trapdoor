// Extended Euclidean Algorithm
// gcd(a, b) = s*a + t*b
pub fn ext_euclid(a: i64, b: i64) -> (i64, i64, i64) {
    let mut r: Vec<i64> = vec![a, b];
    let mut s: Vec<i64> = vec![1, 0];
    let mut t: Vec<i64> = vec![0, 1];
    let mut k: usize = 2;
    while r[k-1] != 0 {
        let q = r[k-2] / r[k-1];
        r.push(r[k-2] % r[k-1]);
        s.push(s[k-2] - q * s[k-1]);
        t.push(t[k-2] - q * t[k-1]);
        k += 1;
    }
    if s[k-2] < 0 {
        s[k-2] += b; 
    }
    (r[k-2], s[k-2], t[k-2])
}
// Modular exponentiation
pub fn mod_exp(mut b: i64, mut e: i64, m: i64) -> i64 {
    let mut result = 1;
    b = b % m;
    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % m;
        }
        b = (b * b) % m;
        e /= 2;
    }
    result
}

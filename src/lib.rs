/// Update a value if the new value is greater than the current value.
///
/// # Arguments
///
/// * `value` - A mutable reference to the value to potentially update
/// * `new_value` - The new value to compare and potentially assign
///
/// # Returns
///
/// Returns `true` if the value was updated, `false` otherwise.
///
/// # Examples
///
/// ```
/// use omochi_gyuhi_atcoder::chmax;
///
/// let mut x = 5;
/// assert_eq!(chmax(&mut x, 10), true);
/// assert_eq!(x, 10);
/// assert_eq!(chmax(&mut x, 3), false);
/// assert_eq!(x, 10);
/// ```
pub fn chmax<T: PartialOrd>(value: &mut T, new_value: T) -> bool {
    if *value < new_value {
        *value = new_value;
        true
    } else {
        false
    }
}

/// Update a value if the new value is smaller than the current value.
///
/// # Arguments
///
/// * `value` - A mutable reference to the value to potentially update
/// * `new_value` - The new value to compare and potentially assign
///
/// # Returns
///
/// Returns `true` if the value was updated, `false` otherwise.
///
/// # Examples
///
/// ```
/// use omochi_gyuhi_atcoder::chmin;
///
/// let mut x = 10;
/// assert_eq!(chmin(&mut x, 5), true);
/// assert_eq!(x, 5);
/// assert_eq!(chmin(&mut x, 8), false);
/// assert_eq!(x, 5);
/// ```
pub fn chmin<T: PartialOrd>(value: &mut T, new_value: T) -> bool {
    if *value > new_value {
        *value = new_value;
        true
    } else {
        false
    }
}

pub struct RollingHash {
    base: u64,
    modulo: u64,
    pow: Vec<u64>,
    hash: Vec<u64>,
}

impl RollingHash {
    pub fn new(s: &[u8], base: u64, modulo: u64) -> Self {
        let n = s.len();
        let mut h = 0;
        let mut p = 1;
        let mut hash = vec![h];
        let mut pow = vec![p];
        for i in 0..n {
            h = h * base % modulo;
            let x = s[i] as u64 + 1;
            h = (h + x) % modulo;
            p = p * base % modulo;
            hash.push(h);
            pow.push(p);
        }
        Self {
            base,
            modulo,
            pow,
            hash,
        }
    }

    pub fn get(&self, l: usize, r: usize) -> u64 {
        if r < l {
            return u64::MAX;
        }
        let hash_r = self.hash[r];
        let hash_l = self.hash[l];
        let len = r - l;
        (self.modulo + hash_r - (hash_l * self.pow[len] % self.modulo)) % self.modulo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_hash_basic() {
        let s = b"hello";
        let rh = RollingHash::new(s, 1000000007, 1000000009);
        
        // Test that we can get hash for valid ranges
        let hash1 = rh.get(0, 5);
        assert_ne!(hash1, u64::MAX); // Should not return error value
        
        // Test invalid range (r < l)
        let hash2 = rh.get(5, 0);
        assert_eq!(hash2, u64::MAX); // Should return error value
    }

    #[test]
    fn test_rolling_hash_substring() {
        let s = b"abcabc";
        let rh = RollingHash::new(s, 1000000007, 1000000009);
        
        // Hash of "abc" at position 0-3 should equal hash of "abc" at position 3-6
        let hash1 = rh.get(0, 3);
        let hash2 = rh.get(3, 6);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_rolling_hash_empty_string() {
        let s = b"";
        let rh = RollingHash::new(s, 1000000007, 1000000009);
        
        // Hash of empty range should be 0
        let hash = rh.get(0, 0);
        assert_eq!(hash, 0);
    }

    #[test]
    fn test_rolling_hash_single_char() {
        let s = b"a";
        let rh = RollingHash::new(s, 1000000007, 1000000009);
        
        let hash = rh.get(0, 1);
        assert_ne!(hash, u64::MAX);
    }

    #[test]
    fn test_chmax() {
        let mut x = 5;
        
        // Update with larger value - should return true and update
        assert_eq!(chmax(&mut x, 10), true);
        assert_eq!(x, 10);
        
        // Try to update with smaller value - should return false and not update
        assert_eq!(chmax(&mut x, 3), false);
        assert_eq!(x, 10);
        
        // Try to update with equal value - should return false and not update
        assert_eq!(chmax(&mut x, 10), false);
        assert_eq!(x, 10);
        
        // Update with larger value again - should return true and update
        assert_eq!(chmax(&mut x, 20), true);
        assert_eq!(x, 20);
    }

    #[test]
    fn test_chmin() {
        let mut x = 10;
        
        // Update with smaller value - should return true and update
        assert_eq!(chmin(&mut x, 5), true);
        assert_eq!(x, 5);
        
        // Try to update with larger value - should return false and not update
        assert_eq!(chmin(&mut x, 15), false);
        assert_eq!(x, 5);
        
        // Try to update with equal value - should return false and not update
        assert_eq!(chmin(&mut x, 5), false);
        assert_eq!(x, 5);
        
        // Update with smaller value again - should return true and update
        assert_eq!(chmin(&mut x, 2), true);
        assert_eq!(x, 2);
    }

    #[test]
    fn test_chmax_chmin_with_floats() {
        let mut x = 5.5;
        
        // Test chmax with floats
        assert_eq!(chmax(&mut x, 10.5), true);
        assert_eq!(x, 10.5);
        assert_eq!(chmax(&mut x, 8.5), false);
        assert_eq!(x, 10.5);
        
        // Test chmin with floats
        assert_eq!(chmin(&mut x, 7.5), true);
        assert_eq!(x, 7.5);
        assert_eq!(chmin(&mut x, 9.5), false);
        assert_eq!(x, 7.5);
    }
}

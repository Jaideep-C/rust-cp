/*
    Sieve of Eratosthenes
    --------------------------------------------------------------
    An algorithm to find all the prime numbers in a given range.
    Time Complexity: O(n log log n)
    Space Complexity: O(n)
    --------------------------------------------------------------
 */
#[allow(dead_code)]
pub fn sieve_of_eratosthenes(upper_limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; upper_limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut primes = Vec::new();
    for num in 2..=upper_limit { // n/2 + n/3 + n/5 + n/7 + n/11 + ...) = n * log log n
        if is_prime[num] {
            primes.push(num);

            let mut multiple = num * num;
            while multiple <= upper_limit { // n/num
                is_prime[multiple] = false;
                multiple += num;
            }
        }
    }

    primes
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_of_eratosthenes_logic() {
        let primes = sieve_of_eratosthenes(10usize.pow(4));
        for p in primes {
            let mut num=1;
            let mut cnt=0;
            while num<=p{
                if p%num==0{
                    cnt+=1;
                }
                num+=1;
            }
            assert_eq!(cnt,2);
        }
    }
    #[test]
    fn test_sieve_of_eratosthenes_time() {
        let primes = sieve_of_eratosthenes(10usize.pow(8));
    }
}
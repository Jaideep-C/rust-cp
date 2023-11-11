use std::io::StdinLock;

const TESTCASE_AVAILABLE: bool = !true;

#[cfg(test)]
mod tests {
    use proptest::proptest;
    use proptest::prelude::*;
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1_000_000))]
        #[test]
        fn input_is_input(input in 0..1000i32) {
            prop_assert_eq!(input,input);
        }
    }
}

fn solve(scanner: &mut Scanner<StdinLock>) {
    println!("{}", scanner.next::<String>());
}

fn main() {
    let mut scanner = Scanner::new(std::io::stdin().lock());
    let t: i64 = if TESTCASE_AVAILABLE { scanner.next() } else { 1 };
    for _ in 0..t {
        solve(&mut scanner);
    }
}


// from https://pastebin.com/raw/qRsQwBQe
pub struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    offset: usize,
}

impl<R: std::io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            offset: 0,
        }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if self.offset < self.buf_str.len() {
                let pos = self.buf_str[self.offset..]
                    .iter()
                    .position(|&c| c == b' ')
                    .unwrap_or(self.buf_str.len() - self.offset);

                let token = std::str::from_utf8(&self.buf_str[self.offset..self.offset + pos])
                    .expect("non utf8")
                    .trim();

                self.offset += pos + 1;
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.offset = 0;
        }
    }
    pub fn next_collection<T, C>(&mut self, n: usize) -> C
        where
            T: std::str::FromStr,
            C: std::iter::FromIterator<T>,
    {
        (0..n).map(|_| self.next()).collect()
    }
}

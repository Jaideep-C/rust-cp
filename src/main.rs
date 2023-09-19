use std::io::StdinLock;

fn solve(scanner: &mut Scanner<StdinLock>){
    println!("{}", scanner.next::<String>());
}

fn main() {
    let mut scanner = Scanner::new(std::io::stdin().lock());
    let t: i64 = scanner.next();
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
}
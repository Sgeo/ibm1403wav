static CHAIN_CHARS_STR: &'static str = "1234567890#@/STUVWXYZ‡,%JKLMNOPQR-$*ABCDEFGHI+.¤";
static CHAIN_CHARS: once_cell::sync::Lazy<Vec<char>> = once_cell::sync::Lazy::new(|| {
    CHAIN_CHARS_STR.chars().collect()
});

const LINE_LENGTH: usize = 132;

#[derive(Debug, Default)]
struct Printer {
    active_hammer: usize,
    active_chain_char: usize,
    subscan: usize,
    scan: usize,
    time: usize
}

impl Printer {
    fn new() -> Printer {
        Printer::default()
    }

    fn advance(&mut self) {
        self.time += 1;
        self.active_hammer += 3;
        self.active_chain_char += 2;
        if self.active_hammer >= LINE_LENGTH {
            self.subscan += 1;
            if self.subscan >= 3 {
                self.subscan = 0;
                self.scan += 1;
            }
            self.active_hammer = self.subscan;
            self.active_chain_char = self.subscan + self.scan;
        }
    }

    fn current_char(&self) -> char {
        CHAIN_CHARS[self.active_chain_char % CHAIN_CHARS.len()]
    }

    fn print(&self, full_line: &str) -> Option<String> {
        //eprintln!("Printing char? {:?}, printer char? {:?}", full_line.chars().collect::<Vec<char>>().get(self.active_hammer), Some(&self.current_char()));
        if full_line.chars().collect::<Vec<char>>().get(self.active_hammer) == Some(&self.current_char()) {
           let mut printing_vec: Vec<char> = vec![' '; LINE_LENGTH];
           printing_vec[self.active_hammer] = self.current_char();
           return Some(String::from_iter(printing_vec));
        } else {
            return None;
        }
    }

    fn finished(&self) -> bool {
        self.time >= LINE_LENGTH * CHAIN_CHARS.len()
    }
}

fn main() -> anyhow::Result<()> {
    let mut printer = Printer::new();
    for line in std::io::stdin().lines() {
        let line = line?;
        let mut prior_time: usize = 0;
        while !printer.finished() {
            let result = printer.print(&line);
            if let Some(printed) = result {
                println!("Printing at time {}, delta = {}", printer.time, printer.time - prior_time);
                println!("{}", printed);
                prior_time = printer.time;
            }
            printer.advance();
        }
    }

    Ok(())
}

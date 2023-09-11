use std::collections::VecDeque;

static CHAIN_CHARS_STR: &'static str = "1234567890#@/STUVWXYZ‡,%JKLMNOPQR-$*ABCDEFGHI+.⌑";
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
    time: u64, // microseconds
    cycles: usize
}

impl Printer {
    fn new() -> Printer {
        Printer {
            time: 0,
            ..Printer::default()
        }
    }

    fn advance(&mut self) {
        self.cycles += 1;
        self.time += 11;
        self.active_hammer += 3;
        self.active_chain_char += 2;
        if self.active_hammer >= LINE_LENGTH {
            eprintln!("Time before sync: {}", self.time);
            self.time += 71; // 555 - 484, sync time
            self.subscan += 1;
            if self.subscan >= 3 {
                eprintln!("Next subscan!");
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
        self.cycles >= LINE_LENGTH * CHAIN_CHARS.len()
    }
}

fn sample_from_time(time: u64) -> u64 {
    44100 * time / 1000000
}

fn main() -> anyhow::Result<()> {
    let mut printer = Printer::new();
    let mut times_microseconds: VecDeque<u64> = VecDeque::new();
    for line in std::io::stdin().lines() {
        let line = line?;
        let mut prior_time: u64 = 0;
        while !printer.finished() {
            let result = printer.print(&line);
            if let Some(printed) = result {
                println!("Printing at time {}, delta = {}", printer.time, printer.time - prior_time);
                println!("{}", printed);
                prior_time = printer.time;
                times_microseconds.push_back(printer.time);
            }
            printer.advance();
        }
    }

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("out.wav", spec)?;

    eprintln!("Number of samples: {}", sample_from_time(*times_microseconds.back().unwrap()));

    for sample in 0..=sample_from_time(*times_microseconds.back().unwrap()) {
        if sample == sample_from_time(*times_microseconds.front().unwrap()) {
            times_microseconds.pop_front();
            writer.write_sample(i16::MAX)?;
        } else {
            writer.write_sample(0)?;
        }
    }

    writer.finalize()?;

    Ok(())
}

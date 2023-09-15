use std::collections::VecDeque;

static CHAIN_CHARS_STR: &'static str = "1234567890#@/STUVWXYZ‡,%JKLMNOPQR-$*ABCDEFGHI+.⌑";
static CHAIN_CHARS_STR_OLD_SIMH: &'static str = "1234567890#@/STUVWXYZ',%JKLMNOPQR-$*ABCDEFGHI&.)";
static CHAIN_CHARS_STR_NEW_SIMH: &'static str = "1234567890#@/STUVWXYZ|,%JKLMNOPQR-$*ABCDEFGHI&.)";
static CHAIN_CHARS: once_cell::sync::Lazy<Vec<char>> = once_cell::sync::Lazy::new(|| {
    CHAIN_CHARS_STR_OLD_SIMH.chars().collect()
});

const LINE_LENGTH: usize = 132;

#[derive(Debug, Default)]
struct LinePrinter {
    line: Vec<char>,
    active_hammer: usize,
    active_chain_char: usize,
    subscan: usize,
    scan: usize,
    time: u64, // microseconds
    cycles: usize
}

impl LinePrinter {
    fn new(line: &str) -> Self {
        Self {
            line: line.chars().collect(),
            time: 0,
            ..Self::default()
        }
    }


    fn current_char(&self) -> char {
        CHAIN_CHARS[self.active_chain_char % CHAIN_CHARS.len()]
    }

}

impl Iterator for LinePrinter {
    type Item = (u64, String);

    fn next(&mut self) -> Option<(u64, String)> {
        let mut result = None;
        while result.is_none() {
            if self.scan > 48 {
                break;
            }
            if self.line.get(self.active_hammer) == Some(&self.current_char()) {
                let mut printing_vec: Vec<char> = vec![' '; LINE_LENGTH];
                printing_vec[self.active_hammer] = self.current_char();
                result = Some((self.time, String::from_iter(printing_vec)));
            }
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
        result
    }
}

fn sample_from_time(time: u64) -> u64 {
    44100 * time / 1000000
}

fn main() -> anyhow::Result<()> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("out.wav", spec)?;


    for line in std::io::stdin().lines() {
        let line = line?;
        let printer = LinePrinter::new(&line);
        let times_microseconds = printer.map(|(time, printed)| {eprintln!("Printed line: {}", printed); time});
        let mut samples: Vec<i16> = vec![0; sample_from_time(1665 * 49) as usize];
        for time in times_microseconds {
            let sample_number = sample_from_time(time);
            samples[sample_number as usize] = i16::MAX;
        }

        for sample in samples {
            writer.write_sample(sample)?;
        }

    }



    writer.finalize()?;

    Ok(())
}

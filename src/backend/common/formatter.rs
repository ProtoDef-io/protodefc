pub struct Formatter {
    pub out: String,
    pub indent_count: u64,
    pub level: u64,
    pub line_started: bool,
}

impl Formatter {

    pub fn line(&mut self) {
        self.out.push_str("\n");
        self.line_started = false;
    }

    fn pad_level(&mut self, level: u64) {
        for _ in 0..level {
            for _ in 0..self.indent_count {
                self.out.push(' ');
            }
        }
    }

    pub fn push(&mut self, string: &str) {
        if self.line_started {
            let level = self.level;
            self.pad_level(level);
        }
        self.line_started = true;
        self.out.push_str(string);
    }

    pub fn indent_down(&mut self) {
        self.level += 1;
    }
    pub fn indent_up(&mut self) {
        self.level -= 1;
    }

}

macro_rules! pf_push {
    ($f:expr, $fmt:expr) => ($f.push($fmt));
    ($f:expr, $fmt:expr, $($arg:tt)*) => ($f.push(&format!($fmt, $($arg)*)));
}

macro_rules! pf_push_line {
    ($f:expr, $fmt:expr) => {
        pf_push!($f, $fmt);
        $f.line();
    };
    ($f:expr, $fmt:expr, $($arg:tt)*) => {
        pf_push!($f, $fmt, $($arg)*);
        $f.line();
    };
}

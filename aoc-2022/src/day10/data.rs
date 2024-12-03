pub struct Cpu {
    cycle: usize,
}

impl Cpu {
    pub const fn new() -> Self {
        Self { cycle: 0 }
    }

    pub fn next_cycle(&mut self) {
        self.cycle += 1;
    }

    pub const fn get_cycle(&self) -> usize {
        self.cycle
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Register {
    value: i64,
}

impl Register {
    pub const fn new(value: i64) -> Self {
        Self { value }
    }

    pub fn add(&mut self, value: i64) {
        self.value += value;
    }

    pub const fn get(&self) -> i64 {
        self.value
    }
}

const PX_DIMMED: char = ' ';
const PX_LIT: char = '#';

pub struct Screen {
    render: String,
    width: usize,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        // I am using already rendered screen as internal representation to save allocations:
        // `width` bytes + newline, repeat `height` times
        let render = (0..height)
            .flat_map(|_| std::iter::repeat(PX_DIMMED).take(width).chain(std::iter::once('\n')))
            .collect();

        Self { render, width }
    }

    pub fn light_px(&mut self, x: usize, y: usize) {
        *self.get_px(x, y) = PX_LIT as u8;
    }

    pub fn into_render(self) -> String {
        self.render
    }

    fn get_px(&mut self, x: usize, y: usize) -> &mut u8 {
        // `y * (width + 1)` to skip `y` screen lines + `y` newline chars
        let idx = y * (self.width + 1) + x;

        // SAFETY: Safe to write into underlying array because self.render
        // consists of either PX_DIMMED, PX_LIT or '\n', which are all ASCII
        unsafe { self.render.as_bytes_mut() }.get_mut(idx).expect("Cycles overflowed!")
    }
}

pub struct VideoSystem {
    reg: Register,
    cpu: Cpu,
    screen: Screen,
    strong_px: usize,
    width: usize,
    total_signal_strength: i64,
}

impl VideoSystem {
    pub fn new(width: usize, height: usize, strong_px: usize) -> Self {
        Self {
            reg: Register::new(1),
            cpu: Cpu::new(),
            screen: Screen::new(width, height),
            strong_px,
            width,
            total_signal_strength: 0,
        }
    }

    pub fn addx(&mut self, value: i64) {
        // Is processed for two cycles, value is added at the end of the second one
        self.cpu.next_cycle();
        self.process_cycle();

        self.cpu.next_cycle();
        self.process_cycle();
        self.reg.add(value);
    }

    pub fn noop(&mut self) {
        self.cpu.next_cycle();
        self.process_cycle();
    }

    pub const fn get_total_signal_strength(&self) -> i64 {
        self.total_signal_strength
    }

    pub fn into_screen_render(self) -> String {
        self.screen.into_render()
    }

    fn process_cycle(&mut self) {
        // 1. Strength calculation
        if self.is_strong_px() {
            self.total_signal_strength += self.get_signal_strength();
        }

        // 2. Light screen pixel if needed
        if self.should_be_lit() {
            let (x, y) = self.get_screen_coords();
            self.screen.light_px(x, y);
        }
    }

    const fn is_strong_px(&self) -> bool {
        self.cpu.get_cycle() % self.width == self.strong_px
    }

    const fn should_be_lit(&self) -> bool {
        let (x, _) = self.get_screen_coords();
        i64::abs_diff(self.reg.get(), x as i64) <= 1
    }

    const fn get_signal_strength(&self) -> i64 {
        (self.cpu.get_cycle() as i64) * self.reg.get()
    }

    const fn get_screen_coords(&self) -> (usize, usize) {
        let through_lines = self.cpu.get_cycle() - 1;
        (through_lines % self.width, through_lines / self.width)
    }
}

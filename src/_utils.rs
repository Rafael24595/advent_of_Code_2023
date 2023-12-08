use std::string::ToString;

pub(crate) enum CONSOLE_COLORS {
    CONSOLE_RESET,
    CONSOLE_SUCESS,
    CONSOLE_FAIL,
    CONSOLE_POWER,
    CONSOLE_RESULT
}

impl CONSOLE_COLORS {

    pub fn as_str(&self) -> &'static str {
        match self {
            CONSOLE_COLORS::CONSOLE_RESET => "\x1b[0m",
            CONSOLE_COLORS::CONSOLE_SUCESS => "\x1b[32m",
            CONSOLE_COLORS::CONSOLE_FAIL => "\x1b[31m",
            CONSOLE_COLORS::CONSOLE_POWER => "\x1b[34m",
            CONSOLE_COLORS::CONSOLE_RESULT => "\x1b[33m",
        }
    }

    pub fn wrap<T: ToString>(&self, string: T) -> String {
        match self {
            CONSOLE_COLORS::CONSOLE_RESET => "\x1b[0m".to_owned() + &string.to_string() + "\x1b[0m",
            CONSOLE_COLORS::CONSOLE_SUCESS => "\x1b[32m".to_owned() + &string.to_string() + "\x1b[0m",
            CONSOLE_COLORS::CONSOLE_FAIL => "\x1b[31m".to_owned() + &string.to_string() + "\x1b[0m",
            CONSOLE_COLORS::CONSOLE_POWER => "\x1b[34m".to_owned() + &string.to_string() + "\x1b[0m",
            CONSOLE_COLORS::CONSOLE_RESULT => "\x1b[33m".to_owned() + &string.to_string() + "\x1b[0m",
        }
    }

}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    return (a * b) / gcd(a, b);
}

pub fn lcm_vector(numbers: Vec<u64>) -> u64 {
    numbers.into_iter().fold(1, |a, b| lcm(a, b))
}
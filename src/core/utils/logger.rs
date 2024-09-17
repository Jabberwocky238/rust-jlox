pub struct Logger;
impl Logger {
    pub fn error(line: usize, message: &str) {
        Self::report(line, "", message);
    }

    pub fn report(line: usize, at: &str, message: &str) {
        println!("[line {}] Error: {} at {}", line, message, at);
        unsafe { 
            HAD_ERROR = true 
        };
    }
}
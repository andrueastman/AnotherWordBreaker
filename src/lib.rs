use lindera::{DictionaryConfig, DictionaryKind, Mode, Penalty, Tokenizer, TokenizerConfig};
use once_cell::sync::Lazy;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(C)]
pub struct BreakResult {
    pub start: u64,
    pub end: u64,
}

static KOREAN_TOKENIZER: Lazy<Tokenizer> = Lazy::new(|| {
    let config = TokenizerConfig {
        dictionary: DictionaryConfig { kind: Some(DictionaryKind::KoDic), path: None },
        mode: Mode::Decompose(Penalty::default()),
        ..TokenizerConfig::default()
    };
    Tokenizer::from_config(config).unwrap()
});

#[no_mangle]
pub extern "C" fn tokenize(input: *const c_char, len: *mut usize) -> *mut BreakResult {
    let c_str = unsafe { CStr::from_ptr(input) };
    let r_str = c_str.to_str().unwrap();
    let mut result = breakStrings(r_str);
    let length = result.len();
    unsafe {
        *len = length;
    }
    let ptr = result.as_mut_ptr();
    std::mem::forget(result); // Prevent Rust from deallocating the memory

    ptr
}

pub fn breakStrings(input: &str) -> Vec<BreakResult>
{
    let mut result:Vec<BreakResult> = Vec::new();
    let segment_iterator = KOREAN_TOKENIZER.tokenize(input).unwrap();
    for token in segment_iterator {
        result.push(BreakResult{ start: token.byte_start as u64, end: token.byte_end as u64})
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let orig = "결정하겠다";
        let result = breakStrings(orig);
    }
}
pub use crate::schema::{Course, CourseModule};
use soroban_sdk::{symbol_short, Bytes, Env, String, Symbol, Vec};

pub fn to_lowercase(env: &Env, s: &String) -> String {
    let len = s.len() as usize;
    let mut buffer = alloc::vec![0u8; len];
    s.copy_into_slice(&mut buffer);
    let mut result_bytes = Bytes::new(env);
    for byte in buffer.iter() {
        if *byte >= b'A' && *byte <= b'Z' {
            result_bytes.push_back(*byte + (b'a' - b'A'));
        } else {
            result_bytes.push_back(*byte);
        }
    }
    let mut result = alloc::vec![0u8; len];
    result_bytes.copy_into_slice(&mut result);
    String::from_bytes(env, &result)
}

pub fn concat_strings(env: &Env, strings: Vec<String>) -> String {
    let mut result = Bytes::new(env);
    let mut total_len = 0;

    for s in strings {
        let s_len = s.len() as usize;
        total_len += s_len;

        let mut buffer = alloc::vec![0u8; s_len];
        s.copy_into_slice(&mut buffer);  
        result.extend_from_slice(&buffer);
    }
    let mut output = alloc::vec![0u8; total_len];
    result.copy_into_slice(&mut output);
    String::from_bytes(env, &output)
}

pub fn u32_to_string(env: &Env, n: u32) -> String {
        // Simple conversion: handle 0 and build digits
        let mut len = 0;

        if n == 0 {
            return String::from_str(env, "0");
        }
        let mut digits = Vec::<u32>::new(env);
        let mut num = n;
        while num > 0 {
            len += 1;
            let digit = (num % 10) as u8;
            digits.push_front((b'0' + digit).into());
            num /= 10;
        }
        let mut bytes = Bytes::new(env);
        for digit in digits.iter() {
            bytes.push_back(digit.try_into().unwrap());
        }

        let mut output = alloc::vec![0u8; len];
        bytes.copy_into_slice(&mut output);
        String::from_bytes(env, &output)
}

pub fn trim(env: &Env, s: &String) -> String {
    // Create a fixed-size buffer for the string's bytes
    let len = s.len() as usize;
    let mut byte_array: [u8; 1024] = [0u8; 1024]; // Adjust size as needed
    if len > byte_array.len() {
        panic!("String too long for fixed-size buffer");
    }
    s.copy_into_slice(&mut byte_array[..len]);

    // Create a Bytes object from the buffer
    let bytes = Bytes::from_slice(env, &byte_array[..len]);

    // Find the first non-whitespace character
    let mut start = 0;
    while start < bytes.len() {
        let mut byte_buffer: [u8; 1] = [0u8; 1];
        bytes.slice(start..start + 1).copy_into_slice(&mut byte_buffer);
        let byte = byte_buffer[0];
        if byte != 32 && byte != 9 {
            break;
        }
        start += 1;
    }

    // Find the last non-whitespace character
    let mut end = bytes.len();
    while end > start {
        let mut byte_buffer: [u8; 1] = [0u8; 1];
        bytes.slice(end - 1..end).copy_into_slice(&mut byte_buffer);
        let byte = byte_buffer[0];
        if byte != 32 && byte != 9 {
            break;
        }
        end -= 1;
    }

    // Create a trimmed Bytes object
    let trimmed_bytes = bytes.slice(start as u32..end as u32);

    let mut output = alloc::vec![0u8; (end - start) as usize];
    trimmed_bytes.copy_into_slice(&mut output);
    String::from_bytes(env, &output)
}


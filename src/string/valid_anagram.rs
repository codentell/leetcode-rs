pub fn valid_anagram(s: String, t: String) -> bool {
    
    if s.len() != t.len() {
        return false
    }

    let mut v: Vec<i32>= vec![0; 26];
    for ch in s.chars() {
        let idx =  (ch as u8 - b'a') as usize;
        println!("{}", idx)
    }

    


    return true;
}

fn main() {
    println!("{}", valid_anagram("anagram".to_string(), "nagaram".to_string()));
}
pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32>  {
    
    let mut result: Vec<i32> = vec![];

    for i in 0..words.len(){
        if words[i].contains(x) {
            result.push(i as i32);
        }
    }
    return result;
}
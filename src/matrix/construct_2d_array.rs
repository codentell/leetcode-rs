pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>>  {

    if m * n != original.len().into().unwrap() {
        return vec![] 
    }

    let mut array = vec![vec![0, n as i32]; (m as i32)];

    for i in 0..original.len() {
        let row = i / n as usize;
        let col = i % n as usize; 

        array[row][col] = original[i];
    } 

    return array;

}
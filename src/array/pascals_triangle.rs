pub fn pascals_triangle(num_rows: i32) -> Vec<Vec<i32>> {

    let mut triangle: Vec<Vec<i32>> = Vec::new();

    for row_num in 0..num_rows as usize {
        let mut row = vec![1; row_num + 1];

        for j in 1..row_num {
            row[j] = triangle[row_num-1][j-1] + triangle[row_num-1][j]
        }

        triangle.push(row);
    }

    return triangle;
}
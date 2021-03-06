//https://rosettacode.org/wiki/Reduced_row_echelon_form#Rust

fn main() {
    let mut matrix_to_reduce: Vec<Vec<f64>> = vec![vec![1.0, 2.0 , -1.0, -4.0], 
                                                vec![2.0, 3.0, -1.0, -11.0],
                                                vec![-2.0, 0.0, -3.0, 22.0]];
    let mut r_mat_to_red = &mut matrix_to_reduce;
    let rr_mat_to_red = &mut r_mat_to_red;
 
    println!("Matrix to reduce:\n{:?}", rr_mat_to_red);
    let reduced_matrix = reduced_row_echelon_form(rr_mat_to_red);
    println!("Reduced matrix:\n{:?}", reduced_matrix);
}
 
fn reduced_row_echelon_form(matrix: &mut DMatrix<f32>) -> DMatrix<f32> {
    let mut matrix_out: DMatrix<f32> = matrix;
    let mut pivot = 0;
    let row_count = result.nrows();
    let column_count = result.ncols();
 
    for r in 0..row_count {
        if column_count <= pivot {
            break;
        }
        let mut i = r;

        while matrix_out[(i, pivot)] == 0.0 {
            i = i+1;
            if i == row_count {
                i = r;
                pivot = pivot + 1;
                if column_count == pivot {
                    pivot = pivot - 1;
                    break;
                }
            }
        }
        for j in 0..row_count {
            let temp = matrix_out[(r,j)];
            matrix_out[r][j] = matrix_out[(i,j)];
            matrix_out[(i,j)] = temp;
        }
        let divisor = matrix_out[(r,pivot)];
        if divisor != 0.0 {
            for j in 0..column_count {
                matrix_out[(r,j)] = matrix_out[(r,j)] / divisor;
            }
        }
        for j in 0..row_count {
            if j != r {
                let hold = matrix_out[(j,pivot)];
                for k in 0..column_count {
                    matrix_out[(j,k)] = matrix_out[(j,k)] - ( hold * matrix_out[(r,k)]);
                }
            }
        }
        pivot = pivot + 1;
    }
    matrix_out;
}
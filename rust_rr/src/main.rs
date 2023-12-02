use ndarray::{Array, Array2};

fn find_first_zero_index(arr: &[i32]) -> Option<usize> {
    arr.iter().position(|&x| x == 0)
}

fn main() {
    const N: usize = 32;
    const K: usize = 20;
    const J: usize = 4;
    let mut a = [[0;N]; K];
    
    println!("{},{}",N,K);
    println!("a = {:?} ", a);
    println!("a[0][0] = {}", a[0][0]);

    for j in 1..=J {
        let mut s: usize = 0;
        for i in (1..=N).rev() {
            //if s >= a.len() {
                // sが配列の範囲外の場合、ループを終了
            //    break;
            //}
            if a[s][i-1] == 0 {
                if let Some(index) = find_first_zero_index(&a[s]) {
                    a[s][index] = i as i32;
            }
            }
            else {
                s = s + 1;
                if a[s][i-1] == 0 {
                    if let Some(index) = find_first_zero_index(&a[s]) {
                        a[s][index] = i as i32;
                    }
                }
            }
        }
    }
    println!("a = {:?}", a);

    for k in 0..=K-1 {
        if a[k][0] == 1 {
            println!("{}", k);
        }
    }

    // let matrix: Array2<i32> = Array::from_shape_vec((K, N), a.concat()).unwrap();
    // let result = matrix.reversed_axes();
    // println!("{:?}", result);
}

// use ndarray::{Array, Array2};
// use plotters::prelude::*;
use csv;
use std::fs;

fn find_first_zero_index(arr: &[i32]) -> Option<usize> {
    arr.iter().position(|&x| x == 0)
}

fn main() {
    let list = [8,16,32,64,128,400,600,1000,2000,3000,5000,7000,8000,10000,20000];
    let mut y_list = [0; 15];
    let mut y_list_log_2 = [0.0; 15];
    for n in 0..15 {
        const K: usize = 1000;
        const J: usize = 10;
        let mut a = vec![vec![0;list[n] as usize]; K];
        
        // println!("{},{}",list[n],K);
        // println!("a = {:?} ", a);
        // println!("a[0][0] = {}", a[0][0]);

        for j in 1..=J {
            let mut s: usize = 0;
            for i in (1..=list[n] as usize).rev() {
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
        // println!("a = {:?}", a);

        let mut b = 1;
        let mut count = 0;
        let mut sum_diff = 0;
        for k in 0..=K-1 {
            if a[k][0] == 1 {
                sum_diff = sum_diff + k - b;
                println!("{}番目の財が,{}番目に配られた.その差は{}",b,k,k-b);
                b = b+1;
            }
            else if a[k][0] != 0{
                count = count + 1;
            }
        }
        println!("{}",sum_diff);
        println!("1以外の人が1の財を取った個数は{}個.",count);
        y_list[n] = count;

        let ll = (list[n] as f64).log2();
        y_list_log_2[n] = ll;
        println!("log_2 N={}", ll);

        println!("今{}回目",n);
    }
    

    // let matrix: Array2<i32> = Array::from_shape_vec((K, N), a.concat()).unwrap();
    // let result = matrix.reversed_axes();
    // println!("{:?}", result);

    println!("{:?}",list);
    println!("{:?}",y_list);
    println!("{:?}",y_list_log_2);

    let file_out = fs::File::options()
        .write(true)
        .create(true)
        .open("incident_list.csv")
        .expect("csvの書き込み失敗");

    let mut wtr = csv::Writer::from_writer(&file_out);

    for nn in 0..15 {
        wtr.serialize([list[nn] as f64, y_list[nn] as f64, y_list_log_2[nn]]).expect("csvエラー");
    }
}

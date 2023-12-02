fn find_first_zero_index(arr: &[i32]) -> Option<usize> {
    arr.iter().position(|&x| x == 0)
}

fn main() {
    let n: i32 = 16;
    let k: i32 = 3;
    let mut a = [[0;16]; 10];
    
    println!("{},{}",n,k);
    println!("a = {:?} ", a);
    println!("a[0][0] = {}", a[0][0]);

    for j in 1..=4 {
        let mut s: usize = 0;
        for i in (1..=16).rev() {
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
}

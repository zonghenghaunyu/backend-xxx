#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    #[test]
    fn it_works() {
        let a1 = vec![0, 0, 2, 0, 10];
        let a2 = vec![1, 0, 0, 0, 0];
        let a3 = vec![1, 2, 0, 99, 0];
        let arr = vec![a1, a2, a3];

        let i = chi_dou_zi(arr);
        chi_dou_zi2(vec![]);
        println!("{}", i)
    }

    fn chi_dou_zi(arr: Vec<Vec<u32>>) -> u32 {
        let mut max = arr.clone();

        let w = arr.len();
        let h = arr[0].len();
        for i in 1..w {
            max[i][0] = max[i - 1][0] + arr[i][0];
        }
        for i in 1..h {
            max[0][i] = max[0][i - 1] + arr[0][i];
        }

        for i in 1..w {
            for j in 1..h {
                max[i][j] = max[i - 1][j].max(max[i][j - 1]) + arr[i][j];
            }
        }

        max[max.len() - 1][max[0].len() - 1]
    }

    fn chi_dou_zi2(arr: Vec<Vec<u32>>) -> Vec<Vec<(usize, usize)>> {
        let mut max = arr.clone();

        let w = arr.len();
        let h = arr[0].len();

        let mut i = 0;
        let mut j = 0;

        let mut stack = VecDeque::<(usize, usize)>::new();
        //压第一个元素入栈
        stack.push_front((i, j));
        while i < w || j < h {
            match stack.pop_front() {
                None => {
                    break;
                }
                Some((a, b)) => {
                    let num = arr[a][b];
                    println!("{num}");
                    j += 1;
                    i += 1;
                }
            }
        }

        for i in 1..w {
            for j in 1..h {
                max[i][j] = max[i - 1][j].max(max[i][j - 1]) + arr[i][j];
            }
        }

        vec![]
    }
}

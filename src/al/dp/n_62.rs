#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let _s = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        let _x = unique_paths(3, 7);
        let x = unique_paths2(3, 7);
        println!("{}", x)
    }
    #[test]
    fn it_works2() {
        // [[0,0,0],[0,1,0],[0,0,0]]
        let x = vec![0, 0, 0];
        let x1 = vec![0, 1, 0];
        let x2 = vec![0, 0, 0];
        let ax = vec![x, x1, x2];
        let a = unique_paths_with_obstacles(ax);
        println!("{}", a)
    }

    // [[1,0]]
    //p63 有石头
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut array = vec![vec![0; n]; m];
        for i in 0..n {
            array[0][i] = 1;
            if obstacle_grid[0][i] == 1 {
                for j in i..n {
                    array[0][j] = 0;
                }
                break;
            }
        }
        for i in 0..m {
            array[i][0] = 1;
            if obstacle_grid[i][0] == 1 {
                for j in i..m {
                    array[j][0] = 0;
                }
                break;
            }
        }
        for i in 1..m {
            for j in 1..n {
                array[i][j] = array[(i - 1)][j] + array[i][(j - 1)];
                if obstacle_grid[i][j] == 1 {
                    array[i][j] = 0;
                }
            }
        }
        array[(m - 1)][(n - 1)]
    }

    fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut array = vec![vec![0; n]; m];
        for x in array.iter_mut() {
            x[0] = 1;
        }
        for x in array.iter_mut() {
            for x in x.iter_mut() {
                *x = 1;
            }
            break;
        }
        for i in 1..m {
            for j in 1..n {
                array[i][j] = array[(i - 1)][j] + array[i][(j - 1)];
            }
        }
        array[(m - 1)][(n - 1)]
    }

    fn unique_paths2(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut array = vec![vec![0; n]; m];
        for i in 0..n {
            array[0][i] = 1;
        }
        for i in 0..m {
            array[i][0] = 1;
        }

        for i in 1..m {
            for j in 1..n {
                array[i][j] = array[(i - 1)][j] + array[i][(j - 1)];
            }
        }
        array[(m - 1)][(n - 1)]
    }
}

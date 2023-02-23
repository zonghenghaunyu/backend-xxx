#[cfg(test)]
mod test {
    //给你一个数组 points ，其中 points[i] = [xi, yi] ，表示第 i 个点在二维平面上的坐标。多个点可能会有 相同 的坐标。

    // 同时给你一个数组 queries ，其中 queries[j] = [xj, yj, rj] ，表示一个圆心在 (xj, yj) 且半径为 rj 的圆。

    // 对于每一个查询 queries[j] ，计算在第 j 个圆 内 点的数目。如果一个点在圆的 边界上 ，我们同样认为它在圆 内 。

    // 请你返回一个数组 answer ，其中 answer[j]是第 j 个查询的答案。

    pub fn count_points1(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries.iter().map(|a|

            // points.iter().fold(0, |acc,b| {
            //     if (b[0] - a[0]) * (b[0] - a[0]) + (b[1] - a[1]) * (b[1] - a[1])  <= a[2] * a[2] {
            //         acc + 1
            //     }else {
            //         acc
            //     }
            // })
            points.iter().filter(|b| (b[0] - a[0]) * (b[0] - a[0]) + (b[1] - a[1]) * (b[1] - a[1])  <= a[2] * a[2]).collect::<Vec<_>>().len() as i32
        ).collect::<Vec<_>>()
    }

    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::<i32>::new();

        for ele in queries {
            let mut curent_num = 0;
            for point in &points {
                if is_in_this(&ele, &point) {
                    curent_num += 1;
                }
            }
            ans.push(curent_num)
        }

        ans
    }
    pub fn is_in_this(area: &Vec<i32>, point: &Vec<i32>) -> bool {
        let x = point[1];
        let y = point[2];
        let x1 = area[1];
        let y1 = area[2];
        let distance = area[3];

        let abs_x = (x - x1).abs();
        let abs_y = (y - y1).abs();
        if abs_x * abs_x + abs_y * abs_y < distance * distance {
            return true;
        }

        false
    }

    #[test]
    fn it_work() {
        print!("aaa")
    }
}

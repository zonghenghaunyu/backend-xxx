#[cfg(test)]
mod test {
    use std::process::id;

    use crate::al;

    /*
    给定一个数组 prices ，它的第 i 个元素 prices[i] 表示一支给定股票第 i 天的价格。
    你只能选择 某一天 买入这只股票，并选择在 未来的某一个不同的日子 卖出该股票。设计一个算法来计算你所能获取的最大利润。
    返回你可以从这笔交易中获取的最大利润。如果你不能获取任何利润，返回 0 。
    示例 1：
    输入：[7,1,5,3,6,4]
    输出：5
    解释：在第 2 天（股票价格 = 1）的时候买入，在第 5 天（股票价格 = 6）的时候卖出，最大利润 = 6-1 = 5 。
         注意利润不能是 7-1 = 6, 因为卖出价格需要大于买入价格；同时，你不能在买入前卖出股票。
    示例 2：
    输入：prices = [7,6,4,3,1]
    输出：0
    解释：在这种情况下, 没有交易完成, 所以最大利润为 0。
    提示：
        1 <= prices.length <= 105
        0 <= prices[i] <= 104

     */
    #[test]
    pub fn is_work() {
        let arr = vec![7, 1, 5, 3, 6,7, 4];
        print!("is work\n{}\n", all_max_profit(arr))
    }


    //calcualte max profit ,before buy you max sell the old stone
    pub fn all_max_profit(prices: Vec<i32>) -> i32{

        let mut pri = prices[0];
        let mut cur_prifit = 0;
        let mut all_prifit = 0;
        //4,3,6,7,3,1,7
        for ele in prices {
            if ele > pri{
                cur_prifit = cur_prifit+ele - pri;
            }else {
                all_prifit += cur_prifit;
                cur_prifit = 0;
            }
            pri = ele;
        }
        all_prifit
    }
    pub fn max_profit1(prices: Vec<i32>) -> i32{

        let mut min = prices[0];
        let mut max_prifit = 0;
        for ele in prices {
            if ele < min {
                min = ele;
            }
            max_prifit = max_prifit.max(ele - min);
        }

        max_prifit
    }

    //动态规划 前i天的最大收益 = max{前i-1天的最大收益，第i天的价格-前i-1天中的最小价格}
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = prices[0];
        let mut last = 0;

        let mut new = 0;
        for idx in 1..prices.len() {
            if prices[idx] < min {
                min = prices[idx]
            };
            new = last.max(prices[idx] - min);
            last = new;
        }
        new
    }
}

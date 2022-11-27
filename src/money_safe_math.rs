
use rust_decimal::prelude::*;

pub fn safe_mul(num1: &str, num2: &str) -> String {
    let u_num1 = Decimal::from_str(num1).unwrap();
    let u_num2 = Decimal::from_str(num2).unwrap();
    let res = u_num1 * u_num2;
    res.round_dp_with_strategy(
        8,
        RoundingStrategy::ToZero
    ).to_string()
}

pub fn safe_div(dividend: &str, divisor: &str) -> String {
    let u_num1 = Decimal::from_str(dividend).unwrap();
    let u_num2 = Decimal::from_str(divisor).unwrap();
    let res = u_num1 / u_num2;
    res.round_dp_with_strategy(
        8, 
        RoundingStrategy::ToZero
    ).to_string()
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_safe_mul() {
        let n1 = "0.50000000";
        let n2 = "2.00000000";
        assert_eq!(safe_mul(n1, n2), "1.00000000");

        let n1 = "1.50000000";
        let n2 = "2.00000000";
        assert_eq!(safe_mul(n1, n2), "3.00000000");

        let n1 = "0.06570000";
        let n2 = "15.30000000";
        assert_eq!(safe_mul(n1, n2), "1.00521000");
    }

    #[test]
    fn test_sage_div() {
        let n1 = "1.00000000";
        let n2 = "2.00000000";
        assert_eq!(safe_div(n1, n2), "0.50");

        let n1 = "16.00000000";
        let n2 = "76.00000000";
        assert_eq!(safe_div(n1, n2), "0.21052631");

        let n1 = "110.00000000";
        let n2 = "68.00000000";
        assert_eq!(safe_div(n1, n2), "1.61764705");
    }

}
use rand::Rng;
pub trait OnesCompletionAdd {
    fn ones_completion_add(self, rhs: Self) -> Self;
}
macro_rules! impl_ones_completion_add {
    ($($t:ty),*) => {
        $(
            impl OnesCompletionAdd for $t {
                fn ones_completion_add(self, rhs: Self) -> Self {
                    let (sum, carry) = self.overflowing_add(rhs);
                    sum + if carry { 1 } else { 0 }
                }
            }
        )*
    };
}
impl_ones_completion_add!(u8, u16);
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ones_completion_add() {
        let zero = 0b_1111_1111_u8;
        assert_eq!(1_u8.ones_completion_add(1), 2);
        assert_eq!(zero.ones_completion_add(5), 5);
        assert_eq!(zero.ones_completion_add(zero), zero);
        assert_eq!((!2_u8).ones_completion_add(3_u8), 1);
        assert_eq!((!2_u8).ones_completion_add(!4_u8), !6_u8);
    }
}
fn check_sum<'a, T>(data: T) -> u16
where
    T: Iterator<Item = &'a u16>,
{
    !data.fold(0, |acc, x| acc.ones_completion_add(*x))
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut data: Vec<u16> = (0..10)
        .map(|idx: usize| if idx == 5 { 0 } else { rng.gen() })
        .collect();
    println!("data: {:?}", data);
    let sum = check_sum(data.iter());
    println!("sum: {:?}", sum);
    data[5] = sum;
    println!("data: {:?}", data);
    let sum = check_sum(data.iter());
    println!("sum: {:?}", sum);
}

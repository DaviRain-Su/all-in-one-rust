pub mod perceptron;

pub use perceptron::Perceptron;

pub fn and(x1: f64, x2: f64) -> bool {
    let p = Perceptron::new(vec![x1, x2], vec![0.5, 0.5], 0.7);
    p.is_active()
}

pub fn nand(x1: f64, x2: f64) -> bool {
    let p = Perceptron::new(vec![x1, x2], vec![-0.5, -0.5], -0.7);
    p.is_active()
}

pub fn or(x1: f64, x2: f64) -> bool {
    let p = Perceptron::new(vec![x1, x2], vec![0.5, 0.5], 0.2);
    p.is_active()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_and() {
        assert_eq!(and(1.0, 1.0), true);
        assert_eq!(and(0.0, 1.0), false);
        assert_eq!(and(1.0, 0.0), false);
        assert_eq!(and(0.0, 0.0), false);
    }

    #[test]
    fn test_xor() {
        assert_eq!(nand(1.0, 1.0), false);
        assert_eq!(nand(0.0, 1.0), true);
        assert_eq!(nand(1.0, 0.0), true);
        assert_eq!(nand(0.0, 0.0), true);
    }

    #[test]
    fn test_or() {
        assert_eq!(or(1.0, 1.0), true);
        assert_eq!(or(0.0, 1.0), true);
        assert_eq!(or(1.0, 0.0), true);
        assert_eq!(or(0.0, 0.0), false);
    }
}

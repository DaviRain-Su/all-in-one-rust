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
        assert!(and(1.0, 1.0));
        assert!(!and(0.0, 1.0));
        assert!(!and(1.0, 0.0));
        assert!(!and(0.0, 0.0));
    }

    #[test]
    fn test_xor() {
        assert!(!nand(1.0, 1.0));
        assert!(nand(0.0, 1.0));
        assert!(nand(1.0, 0.0));
        assert!(nand(0.0, 0.0));
    }

    #[test]
    fn test_or() {
        assert!(or(1.0, 1.0));
        assert!(or(0.0, 1.0));
        assert!(or(1.0, 0.0));
        assert!(!or(0.0, 0.0));
    }
}

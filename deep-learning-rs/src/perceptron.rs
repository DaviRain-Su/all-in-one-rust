/// Perceptron
pub struct Perceptron {
    /// Weights
    pub weights: Vec<f64>,
    /// Inputs
    pub inputs: Vec<f64>,
    /// threshold
    pub threshold: f64,
}

impl Perceptron {
    pub fn new(inputs: Vec<f64>, weights: Vec<f64>, threshold: f64) -> Perceptron {
        assert_eq!(inputs.len(), weights.len());
        Perceptron {
            inputs,
            weights,
            threshold,
        }
    }

    pub fn is_active(&self) -> bool {
        assert_eq!(self.weights.len(), self.inputs.len());
        let mut sum = 0.0;
        for i in 0..self.inputs.len() {
            sum += self.inputs[i] * self.weights[i];
        }
        sum + (-self.threshold) > 0.0
    }
}

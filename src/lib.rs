pub struct Perceptron {
    weights: Vec<f64>,
    bias: f64
}

impl Perceptron {
    pub fn new(weights: Vec<f64>, bias: f64) -> Self {
        Perceptron { weights, bias }
    }
    
    pub fn execute(&mut self, f: impl Fn(Vec<f64>, f64) -> Perceptron) {
        let perceptron = f(self.weights.clone(), self.bias);
        *self = perceptron
    }
}

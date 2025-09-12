pub struct Perceptron {
    values: Vec<Vec<f64>>,
    weights: Vec<f64>,
    bias: f64,
    eta: f64
}

impl Perceptron {
    pub fn new(values: Vec<Vec<f64>>, weights: Vec<f64>, bias: f64, eta: f64) -> Self {
        Perceptron { values, weights, bias, eta }
    }
    
    pub fn execute(&mut self, f: impl Fn(Vec<Vec<f64>>, Vec<f64>, f64, f64) -> Perceptron) {
        let perceptron = f(
            self.values.clone(), 
            self.weights.clone(),
            self.bias, 
            self.eta
        );
        self.weights = perceptron.weights;
        self.bias = perceptron.bias;
    }
}

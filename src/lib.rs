use std::fmt;

pub struct Perceptron {
    values: Vec<Vec<f64>>,
    solutions: Vec<u8>,
    weights: Vec<f64>,
    bias: f64,
    eta: f64
}

impl Perceptron {
    pub fn new(values: Vec<Vec<f64>>, solutions: Vec<u8>, weights: Vec<f64>, bias: f64, eta: f64) -> Self {
        Perceptron { values, solutions, weights, bias, eta }
    }
    
    pub fn execute(&mut self, f: impl Fn(Vec<Vec<f64>>, Vec<u8>, Vec<f64>, f64, f64) -> (Vec<f64>, f64)) {
        let new_params = f(
            self.values.clone(), 
            self.solutions.clone(),
            self.weights.clone(),
            self.bias, 
            self.eta
        );
        self.weights = new_params.0;
        self.bias = new_params.1;
    }
}

impl fmt::Debug for Perceptron {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Perceptron")
            .field("values", &self.values)
            .field("solutions", &self.solutions)
            .field("weights", &self.weights)
            .field("bias", &self.bias)
            .field("eta", &self.eta)
            .finish()
    }
}

impl Clone for Perceptron {
    fn clone(&self) -> Self {
        Perceptron {
            values: self.values.clone(),
            solutions: self.solutions.clone(),
            weights: self.weights.clone(),
            bias: self.bias,
            eta: self.eta
        }
    }
}

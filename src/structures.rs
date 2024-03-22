use nalgebra::DMatrix;
use crate::activation_functions::ActivationFunction;

pub struct Edge {
    value: f64
}



#[derive(Debug)]
pub struct Layer<T: ActivationFunction<f64>> {
    activation_function: T,
    edges: Vec<f64>
}


pub struct Network<T: ActivationFunction<f64>> {
    layers: Vec<Layer<T>>,
    edges: Vec<DMatrix<f64>>
}

impl<T: ActivationFunction<f64>> Network<T> {
    fn new(num_of_edges: Vec<u16>) -> Self {
        let mut network = Network {
            layers: vec![],
            edges: vec![]

        };
        for num in num_of_edges {
            let layer = Layer {
                edges: vec![0.0; num as usize],
                activation_function: T

            }
        };
    }
}





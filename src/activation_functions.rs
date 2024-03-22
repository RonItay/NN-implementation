use libm::exp;
pub struct Sigmoid {}
impl ActivationFunction<f64> for Sigmoid {
    fn f(input: f64) -> f64 {
        1.0 / (exp(input) + 1.0)
    }

    fn df(input: f64) -> f64 {
        // dsigmoid(x)/dx = sigmoid(x)[1-sigmoid(x)], here we enter the sigmoid value
        input * (1.0 - input)
    }
}

pub trait ActivationFunction<T> {
    fn f(input: T) -> T;
    fn df(input: T) -> T;
}
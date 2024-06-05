use burn::prelude::*;
use nn::{conv::Conv2d, pool::AdaptiveAvgPool2d, Dropout, Linear, Relu};

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    conv1: Conv2d<B>,
    conv2: Conv2d<B>,
    pool: AdaptiveAvgPool2d,
    droput: Dropout,
    linear1: Linear<B>,
    linear2: Linear<B>,
    activation: Relu,
}

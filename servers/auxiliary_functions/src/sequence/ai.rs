use crate::sequence::models::{Sequence, Range};

use linfa::prelude::*;
use linfa_linear::{LinearRegression, FittedLinearRegression};
use ndarray::Array2;

pub struct AiModel {
    model: Option<FittedLinearRegression<f64>>,
    training_range: Range,
}

impl AiModel {
    pub fn new(training_range: Range) -> Self {
        AiModel {
            model: None,
            training_range,
        }
    }

    pub fn train(&mut self, input_sequence: &Vec<f64>) {
        let num_samples = (input_sequence.len() as u64 - self.training_range.from) as usize;
        let num_features = (self.training_range.to - self.training_range.from) as usize;
        let mut x_train = Array2::<f64>::zeros((num_samples, num_features));
        let mut y_train = Array2::<f64>::zeros((num_samples, 1));

        for i in 0..num_samples {
            for j in 0..num_features {
                x_train[[i, j]] = input_sequence[(i + j) as usize];
            }
            y_train[[i, 0]] = input_sequence[(i + num_features) as usize];
        }

        // Create the DatasetBase
        let dataset = Dataset::new(x_train, y_train);

        // Fit the model using the DatasetBase
        let model = LinearRegression::default();
        self.model = Some(model.fit(&dataset).expect("Failed to fit model"));
    }

    pub fn predict(&self, input_sequence: &Vec<f64>) -> Vec<f64> {
        let num_features = (self.training_range.to - self.training_range.from) as usize;
        let mut predictions = Vec::new();

        if let Some(ref fitted_model) = self.model {
            for i in 0..(input_sequence.len() as u64 - self.training_range.to + 1) as usize {
                let x_pred = Array2::from_shape_vec(
                    (1, num_features),
                    input_sequence[i..i + num_features].to_vec(),
                )
                .expect("Failed to create prediction array");

                let y_pred = fitted_model.predict(&x_pred);
                predictions.push(y_pred[[0]]);
            }
        }

        predictions
    }
}

pub struct Ai<'a> {
    input_sequence: Box<&'a dyn Sequence<f64>>,
    training_range: Range,
    prediction: Vec<f64>,
    model: AiModel,
}

impl<'a> Ai<'a> {
    pub fn new(input_sequence: Box<&'a dyn Sequence<f64>>, training_range: Range) -> Self {
        let mut ai_model = AiModel::new(training_range.clone());

        // Convert the input sequence into a Vec<f64> for training
        let input_data = (0..training_range.to as usize)
            .filter_map(|i| input_sequence.k_th(i))
            .collect::<Vec<f64>>();

        // Train the model
        ai_model.train(&input_data);

        // Predict the sequence
        let prediction = ai_model.predict(&input_data);

        Ai {
            input_sequence,
            training_range,
            prediction,
            model: ai_model,
        }
    }
}

impl<'a> Sequence<f64> for Ai<'a> {
    fn name(&self) -> String {
        format!(
            "AI Sequence based on {} elements of input sequence {} with {} predictions",
            self.training_range.to - self.training_range.from,
            self.input_sequence.name(),
            self.prediction.len()
        )
    }

    fn start(&self) -> f64 {
        self.prediction[0]
    }

    fn k_th(&self, k: usize) -> Option<f64> {
        if k < self.prediction.len() {
            Some(self.prediction[k])
        } else {
            None
        }
    }

    fn contains(&self, value: f64) -> bool {
        self.prediction.contains(&value)
    }
}
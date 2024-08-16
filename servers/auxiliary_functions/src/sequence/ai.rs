use crate::sequence::models::Sequence;

use linfa::prelude::*;
use linfa_linear::LinearRegression;
use ndarray::Array2;

pub struct AiModel {
    model: LinearRegression<f64, ndarray::Dim<[usize; 2]>>,
    training_range: usize,
}

impl AiModel {
    pub fn new(training_range: usize) -> Self {
        let model = LinearRegression::default();
        AiModel { model, training_range }
    }

    pub fn train(&mut self, input_sequence: &Vec<f64>) {
        // Prepare training data
        let num_samples = input_sequence.len() - self.training_range;
        let mut x_train = Array2::<f64>::zeros((num_samples, self.training_range));
        let mut y_train = Array2::<f64>::zeros((num_samples, 1));

        for i in 0..num_samples {
            for j in 0..self.training_range {
                x_train[[i, j]] = input_sequence[i + j];
            }
            y_train[[i, 0]] = input_sequence[i + self.training_range];
        }

        // Fit the model
        self.model = self.model.fit(&x_train, &y_train).expect("Failed to fit model");
    }

    pub fn predict(&self, input_sequence: &Vec<f64>) -> Vec<f64> {
        let mut predictions = Vec::new();

        for i in 0..self.training_range {
            let x_pred = Array2::from_shape_vec((1, self.training_range), input_sequence[i..i+self.training_range].to_vec())
                .expect("Failed to create prediction array");

            let y_pred = self.model.predict(&x_pred);
            predictions.push(y_pred[[0, 0]]);
        }

        predictions
    }
}

pub struct Ai<'a> {
    input_sequence: Box<&'a dyn Sequence<f64>>,
    training_range: usize,
    prediction: Vec<f64>,
    model: AiModel,
}

impl<'a> Ai<'a> {
    pub fn new(input_sequence: Box<&'a dyn Sequence<f64>>, training_range: usize) -> Self {
        let mut ai_model = AiModel::new(training_range);

        // Convert the input sequence into a Vec<f64> for training
        let input_data = (0..training_range)
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



//impl<'a> Ai<'a> {
//    pub fn new(input_sequence: Box<&'a dyn Sequence<f64>>, training_range: usize) -> Self {
//        let prediction = Ai::prediction(&input_sequence, training_range);
//        Ai {
//            input_sequence,
//            training_range,
//            prediction,
//        }
//    }
//
//    fn prediction(input_sequence: &Box<&'a dyn Sequence<f64>>, training_range: usize) -> Vec<f64> {
//        let mut predictions = Vec::new();
//        let mut previous_values = Vec::new();
//
//        for i in 0..training_range {
//            if let Some(value) = input_sequence.k_th(i) {
//                previous_values.push(value);
//            } else {
//                break;
//            }
//        }
//
//        for _ in 0..training_range {
//
//        }
//
//        predictions
//    }
//}

impl<'a> Sequence<f64> for Ai<'a> {
    fn name(&self) -> String {
        format!(
            "AI Sequence based on {} elements of input sequence {} with {} predictions",
            self.training_range,
            self.input_sequence.name(),
            self.prediction.len()
        )
    }

    fn start(&self) -> f64 {
        self.prediction[0]
    }

    fn k_th(&self, k: usize) -> Option<f64> {
        let prediction_len = self.training_range;
        if k < prediction_len {
            Some(self.prediction[k])
        } else {
            None
        }
    }

    fn contains(&self, value: f64) -> bool {
        self.prediction.contains(&value)
    }
}
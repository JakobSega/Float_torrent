use crate::sequence::models::Sequence;
use linfa::prelude::*;
use linfa_linear::{LinearRegression, FittedLinearRegression};
use ndarray::Array2;

pub struct AiModel {
    model: Option<FittedLinearRegression<f64>>,
}

impl AiModel {
    pub fn new() -> Self {
        AiModel {
            model: None,
        }
    }

    pub fn train(&mut self, input_data: &[Option<f64>]) {
        let num_samples = input_data.len() - 1; // Adjusted for simplicity
        let num_features = 1; // For simplicity; adjust if needed based on your actual features

        let mut x_train = Array2::<f64>::zeros((num_samples, num_features));
        let mut y_train = Array2::<f64>::zeros((num_samples, 1));

        for i in 0..num_samples {
            x_train[[i, 0]] = input_data[i].unwrap_or_default();
            y_train[[i, 0]] = input_data[i + 1].unwrap_or_default(); // Simple example
        }

        let dataset = Dataset::new(x_train, y_train);
        let model = LinearRegression::default();
        self.model = Some(model.fit(&dataset).expect("Failed to fit model"));
    }

    pub fn predict(&self, input_data: &[Option<f64>], num_predictions: usize) -> Vec<Option<f64>> {
        let mut predictions = Vec::with_capacity(num_predictions);

        if let Some(ref fitted_model) = self.model {
            for i in 0..(input_data.len() - 1) {
                let x_pred = Array2::from_shape_vec(
                    (1, 1), // Adjust shape if needed
                    vec![input_data[i].unwrap_or_default()],
                ).expect("Failed to create prediction array");

                let y_pred = fitted_model.predict(&x_pred);
                predictions.push(Some(y_pred[[0]]));

                if predictions.len() >= num_predictions {
                    break;
                }
            }

            // If there are fewer predictions than requested, fill the rest with `None`
            while predictions.len() < num_predictions {
                predictions.push(None);
            }
        } else {
            // If the model is not trained, return `None` for all requested predictions
            predictions.extend(vec![None; num_predictions]);
        }

        predictions
    }
}



pub struct Ai {
    pub input_sequence: Vec<Option<f64>>,
    pub prediction: Vec<Option<f64>>,
    pub model: AiModel,
}

impl Ai {
    pub fn new(input_sequence: Vec<Option<f64>>, num_predictions: usize) -> Self {
        let mut ai_model = AiModel::new();

        // Train the model with the input sequence
        ai_model.train(&input_sequence);

        // Get predictions based on the input sequence and number of predictions
        let prediction = ai_model.predict(&input_sequence, num_predictions);

        Ai {
            input_sequence,
            prediction,
            model: ai_model,
        }
    }
}

impl Sequence<f64> for Ai {
    fn name(&self) -> String {
        format!(
            "AI Sequence with {} elements and {} predictions",
            self.input_sequence.len(),
            self.prediction.len()
        )
    }

    fn start(&self) -> f64 {
        self.prediction.get(0).copied().flatten().unwrap_or_default()
    }

    fn k_th(&self, k: usize) -> Option<f64> {
        self.prediction.get(k).and_then(|opt| *opt)
    }

    fn contains(&self, value: f64) -> bool {
        self.prediction.iter().any(|opt| opt == &Some(value))
    }
}

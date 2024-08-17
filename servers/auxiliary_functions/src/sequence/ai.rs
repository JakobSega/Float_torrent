use crate::sequence::models::Sequence;
use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use smartcore::linear::linear_regression::LinearRegression as SmartcoreLinearRegression;
//use ndarray::Array2;

pub struct AiModel {
    model: Option<SmartcoreLinearRegression<f64, DenseMatrix<f64>>>,
}

impl AiModel {
    pub fn new() -> Self {
        AiModel {
            model: None,
        }
    }

    pub fn train(&mut self, input_data: &[Option<f64>]) {
        let num_samples = input_data.len() - 1;
        let num_features = 1;

        let mut x_train = Vec::with_capacity(num_samples * num_features);
        let mut y_train = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            x_train.push(input_data[i].unwrap_or_default());
            y_train.push(input_data[i + 1].unwrap_or_default());
        }

        // Convert x_train to a DenseMatrix
        let x_matrix = DenseMatrix::from_2d_vec(&x_train.iter().map(|&v| vec![v]).collect::<Vec<_>>());

        // y_train is already a vector, no need to convert it
        let y_matrix = y_train.clone();

        // Fit the model
        let model = SmartcoreLinearRegression::fit(&x_matrix, &y_matrix, Default::default())
            .expect("Failed to fit the model");

        self.model = Some(model);
    }

    pub fn predict(&self, input_data: &[Option<f64>], num_predictions: usize) -> Vec<Option<f64>> {
        let mut predictions = Vec::with_capacity(num_predictions);

        if let Some(ref fitted_model) = self.model {
            for i in 0..input_data.len() {
                let x_pred = DenseMatrix::from_2d_vec(&vec![vec![input_data[i].unwrap_or_default()]]);
                let y_pred = fitted_model.predict(&x_pred).expect("Failed to predict");

                predictions.push(Some(y_pred[0])); // y_pred is a vector, extract the first element

                if predictions.len() >= num_predictions {
                    break;
                }
            }

            while predictions.len() < num_predictions {
                predictions.push(None);
            }
        } else {
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

        ai_model.train(&input_sequence);

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

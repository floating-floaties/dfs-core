use csv::ReaderBuilder;
use flate2::read::GzDecoder;
use linfa::composing::MultiClassModel;
use linfa::prelude::*;
use linfa_svm::{error::Result, Svm};
use ndarray::{Array2, ArrayBase, Ix, Ix2, OwnedRepr, s};
use ndarray_csv::Array2Reader;

struct SvmOps {
    csv_gz_path: String,
    feature_names: Vec<String>,
    target_col_idx: Ix,
}

type SvmTrainOutput = (
    MultiClassModel<ArrayBase<OwnedRepr<f64>, Ix2>, usize>,
    ConfusionMatrix<usize>
);

fn array_from_buf(buf: &[u8]) -> Array2<f64> {
    // unzip file
    let file = GzDecoder::new(buf);
    // create a CSV reader with headers and `,` as delimiter
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_reader(file);

    // extract ndarray
    reader.deserialize_array2_dynamic().unwrap()
}

fn array_from_buf_str(buf: &[u8]) -> Array2<String> {
    // unzip file
    let file = GzDecoder::new(buf);
    // create a CSV reader with headers and `,` as delimiter
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_reader(file);

    // extract ndarray
    reader.deserialize_array2_dynamic().unwrap()
}


fn train(opts: SvmOps) -> Result<SvmTrainOutput> {
    let data = std::fs::read(opts.csv_gz_path).unwrap();
    let array = array_from_buf(&data[..]);

    let (data, targets) = (
        array.slice(s![.., ..opts.target_col_idx]).to_owned(),
        array.column(opts.target_col_idx).to_owned(),
    );

    let feature_names = opts.feature_names;

    let dataset: DatasetBase<ArrayBase<OwnedRepr<f64>, Ix2>, _> = Dataset::new(data, targets)
        .map_targets(|x| *x as usize)
        .with_feature_names(feature_names);
    let (train, valid) = dataset.split_with_ratio(0.9);

    println!(
        "Fit SVM classifier with #{} training points",
        train.nsamples()
    );

    let params = Svm::<_, Pr>::params()
        //.pos_neg_weights(5000., 500.)
        .gaussian_kernel(30.0);

    let model = train
        .one_vs_all()?
        .into_iter()
        .map(|(l, x)| (l, params.fit(&x).unwrap()))
        .collect::<MultiClassModel<_, _>>();

    let pred = model.predict(&valid);

    // create a confusion matrix
    let cm = pred.confusion_matrix(&valid)?;

    // Print the confusion matrix
    println!("{:?}", cm);

    // Calculate the accuracy and Matthew Correlation Coefficient (cross-correlation between
    // predicted and targets)
    println!("accuracy {}, MCC {}", cm.accuracy(), cm.mcc());

    Ok((model, cm))
}

#[cfg(test)]
mod svm_tests {
    use crate::ml::svm::*;

    #[test]
    fn usage() {
        let feature_names = vec![
            "fixed acidity",
            "volatile acidity",
            "citric acid",
            "residual sugar",
            "chlorides",
            "free sulfur dioxide",
            "total sulfur dioxide",
            "density",
            "pH",
            "sulphates",
            "alcohol",
        ];
        let ops = SvmOps {
            csv_gz_path: String::from("./winequality-red.csv.gz"),
            feature_names: feature_names.iter().map(|v| v.to_string()).collect(),
            target_col_idx: 11,
        };

        let (_model, _cm) = train(ops).unwrap();

    }

    #[test]
    fn intents() {
        let feature_names = vec![
            "text",
            "intent",
        ];
        let ops = SvmOps {
            csv_gz_path: String::from("./intents.csv.gz"),
            feature_names: feature_names.iter().map(|v| v.to_string()).collect(),
            target_col_idx: 1,
        };

        let (_model, _cm) = train(ops).unwrap();

    }
}
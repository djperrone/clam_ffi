#![allow(dead_code)]
#![allow(unused_variables)]

use abd_clam::{core::dataset, rnn, Cakes, PartitionCriteria, VecDataset};
use distances::{
    self,
    number::{Float, UInt},
    Number,
};
use std::f32::EPSILON;

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]

pub enum DistanceMetric {
    Euclidean,
    EuclideanSQ,
    Manhattan,
    L3Norm,
    L4Norm,
    Chebyshev,
    Cosine,
    Canberra,
    NeedlemanWunsch,
    Levenshtein,
}

/// Generate a dataset from the given data.
pub fn gen_dataset_from<T: Number, U: Number>(
    data: Vec<Vec<T>>,
    metric: fn(&Vec<T>, &Vec<T>) -> U,
) -> VecDataset<Vec<T>, U> {
    let name: String = "test".to_string();
    VecDataset::new(name, data, metric, false)
}

fn gen_f32_dataset() -> VecDataset<Vec<f32>, f32> {
    let data = vec![vec![0., 1., 2., 3.]];
    let dataset = VecDataset::new("test".to_string(), data, euclidean, false);
    return dataset;
}

fn gen_str_dataset() -> VecDataset<String, u16> {
    let data = vec![
        "test1".to_string(),
        "test2".to_string(),
        "test3".to_string(),
    ];
    let dataset = VecDataset::new("test".to_string(), data, levenshtein, false);
    return dataset;
}

fn strings(cardinality: usize, alphabet: &str, metric: fn(&String, &String) -> u16) {
    let seed = 42;
    let seq_len = 100;

    let data = symagen::random_data::random_string(cardinality, seq_len, seq_len, alphabet, seed);
    let data = VecDataset::new("test".to_string(), data.clone(), metric, false);
    let cakes = Cakes::new(data, Some(42), &PartitionCriteria::default());

    let num_queries = 10;
    let queries =
        symagen::random_data::random_string(num_queries, seq_len, seq_len, alphabet, seed + 1);
    let queries = (0..num_queries).map(|i| &queries[i]).collect::<Vec<_>>();

    // check_search_quality(&queries, &cakes, &[1, 5, 10], &[1, 5, 10]);
}

//TODO: Make generic for strings as well
pub fn from_enum<T: Number, U: Number>(metric: DistanceMetric) -> fn(&Vec<T>, &Vec<T>) -> U {
    match metric {
        // DistanceMetric::Euclidean => euclidean,
        // DistanceMetric::EuclideanSQ => euclidean_sq,
        // DistanceMetric::Manhattan => manhattan,
        // DistanceMetric::L3Norm => l3_norm,
        // DistanceMetric::L4Norm => l4_norm,
        // DistanceMetric::Chebyshev => chebyshev,
        // DistanceMetric::Cosine => cosine,
        // DistanceMetric::Canberra => canberra,

        // DistanceMetric::NeedlemanWunsch => needleman,
        // "hamming" => hamming,
        // "jaccard" => jaccard,
        _ => panic!("Distance {:?} is not implemented in clam.", metric),
    }
}

/// Euclidean distance between two vectors.
pub fn euclidean<T: Number, F: Float>(x: &Vec<T>, y: &Vec<T>) -> F {
    distances::vectors::euclidean(x, y)
}

/// Euclidean distance between two vectors.
pub fn euclidean_sq<T: Number>(x: &Vec<T>, y: &Vec<T>) -> T {
    distances::vectors::euclidean_sq(x, y)
}

// /// Euclidean distance between two vectors.
// pub fn euclidean_sq<T: Number>(x: &Vec<T>, y: &Vec<T>) -> T {
//     distances::vectors::euclidean_sq(x, y)
// }
// /// Euclidean distance between two vectors.
// pub fn euclidean<T: Number, F: Float>(x: &Vec<T>, y: &Vec<T>) -> F {
//     distances::vectors::euclidean(x, y)
// }

// Hamming distance between two Strings.
pub fn hamming<T: UInt>(x: &String, y: &String) -> T {
    distances::strings::hamming(x, y)
}

/// Levenshtein distance between two Strings.
pub fn levenshtein<T: UInt>(x: &String, y: &String) -> T {
    distances::strings::levenshtein(x, y)
}

/// Needleman-Wunsch distance between two Strings.
pub fn needleman_wunsch<T: UInt>(x: &String, y: &String) -> T {
    distances::strings::needleman_wunsch::nw_distance(x, y)
}

pub fn manhattan<T: Number>(x: &Vec<T>, y: &Vec<T>) -> T {
    distances::vectors::manhattan(x, y)
}
pub fn l3_norm<T: Number, F: Float>(x: &Vec<T>, y: &Vec<T>) -> F {
    distances::vectors::l3_norm(x, y)
}
pub fn l4_norm<T: Number, F: Float>(x: &Vec<T>, y: &Vec<T>) -> F {
    distances::vectors::l3_norm(x, y)
}
pub fn chebyshev<T: Number>(x: &Vec<T>, y: &Vec<T>) -> T {
    distances::vectors::chebyshev(x, y)
}

pub fn cosine<T: Number, F: Float>(x: &Vec<T>, y: &Vec<T>) -> F {
    distances::vectors::cosine(x, y)
}
pub fn canberra<T: Number, F: Float>(x: &Vec<T>, y: &Vec<T>) -> F {
    distances::vectors::canberra(x, y)
}

// pub fn from_name(name: &str) -> fn(&[f32], &[f32]) -> f32 {
//     match name {
//         "euclidean" => euclidean,
//         "euclidean_sq" => euclidean_sq,
//         "manhattan" => manhattan,
//         "cosine" => cosine,
//         // "hamming" => hamming,
//         // "jaccard" => jaccard,
//         _ => panic!("Distance {name} is not implemented in clam."),
//     }
// }

// #[allow(clippy::type_complexity)]
// pub const METRICS: &[(&str, fn(&[f32], &[f32]) -> f32)] = &[
//     ("euclidean", euclidean),
//     ("euclidean_sq", euclidean_sq),
//     ("manhattan", manhattan),
//     // ("cosine", cosine),
// ];

// #[inline(always)]
// pub fn euclidean<T: Number>(x: &Vec<T>, y: &Vec<T>) -> T {
//     euclidean_sq(x, y).sqrt()
// }

// #[inline(always)]
// pub fn euclidean_sq<T: Number>(x: &Vec<T>, y: &Vec<T>) -> T {
//     x.iter().zip(y.iter()).map(|(&a, &b)| (a - b).powi(2)).sum()
// }

// pub fn euclidean_sq_vec<T: Number>(x: &Vec<T>, y: &Vec<T>) -> T {
//     euclidean_sq(x, y)
//     // x.iter()
//     //     .zip(y.iter())
//     //     .map(|(a, b)| a - b)
//     //     .map(|v| v * v)
//     //     .sum::<f32>()
//     //     .sqrt()
// }

// #[inline(always)]
// pub fn manhattan<T: Number>(x: &Vec<T>, y: &Vec<T>) -> T {
//     x.iter().zip(y.iter()).map(|(&a, &b)| (a - b).abs()).sum()
// }

// #[inline(always)]
// pub fn cosine<T: Number>(x: &Vec<T>, y: &Vec<T>) -> T {
//     let [xx, yy, xy] = x
//         .iter()
//         .zip(y.iter())
//         .fold([0.; 3], |[xx, yy, xy], (&a, &b)| {
//             [xx + a * a, yy + b * b, xy + a * b]
//         });

//     if xx <= EPSILON || yy <= EPSILON || xy <= EPSILON {
//         1.
//     } else {
//         let d = 1. - xy / (xx * yy).sqrt();
//         if d < EPSILON {
//             0.
//         } else {
//             d
//         }
//     }
// }

#![allow(dead_code)]
#![allow(unused_variables)]

use distances::{self, number::UInt};
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

//TODO: Make generic for strings as well
pub fn from_enum(metric: DistanceMetric) -> fn(&Vec<f32>, &Vec<f32>) -> f32 {
    match metric {
        DistanceMetric::Euclidean => euclidean,
        DistanceMetric::EuclideanSQ => euclidean_sq,
        DistanceMetric::Manhattan => manhattan,
        DistanceMetric::L3Norm => l3_norm,
        DistanceMetric::L4Norm => l4_norm,
        DistanceMetric::Chebyshev => chebyshev,
        DistanceMetric::Cosine => cosine,
        DistanceMetric::Canberra => canberra,

        // DistanceMetric::NeedlemanWunsch => needleman,
        // "hamming" => hamming,
        // "jaccard" => jaccard,
        _ => panic!("Distance {:?} is not implemented in clam.", metric),
    }
}

// lp_norms
pub fn euclidean(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    return distances::vectors::euclidean(&x.to_vec(), &y.to_vec());
}
pub fn euclidean_sq(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    return distances::vectors::euclidean_sq(&x.to_vec(), &y.to_vec());
}
pub fn manhattan(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    return distances::vectors::manhattan(&x.to_vec(), &y.to_vec());
}
pub fn l3_norm(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    return distances::vectors::l3_norm(&x.to_vec(), &y.to_vec());
}
pub fn l4_norm(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    return distances::vectors::l3_norm(&x.to_vec(), &y.to_vec());
}
pub fn chebyshev(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    return distances::vectors::chebyshev(&x.to_vec(), &y.to_vec());
}

pub fn cosine(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    return distances::vectors::cosine(&x.to_vec(), &y.to_vec());
}
pub fn canberra(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    return distances::vectors::canberra(&x.to_vec(), &y.to_vec());
}

//Needleman Wunsch
pub fn nw_distance(x: &str, y: &str) -> u32 {
    return distances::strings::nw_distance(x, y);
}

//levenshtein
pub fn levenshtein(x: &str, y: &str) -> u32 {
    return distances::strings::levenshtein(x, y);
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
// pub fn euclidean(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
//     euclidean_sq(x, y).sqrt()
// }

// #[inline(always)]
// pub fn euclidean_sq(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
//     x.iter().zip(y.iter()).map(|(&a, &b)| (a - b).powi(2)).sum()
// }

// pub fn euclidean_sq_vec(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
//     euclidean_sq(x, y)
//     // x.iter()
//     //     .zip(y.iter())
//     //     .map(|(a, b)| a - b)
//     //     .map(|v| v * v)
//     //     .sum::<f32>()
//     //     .sqrt()
// }

// #[inline(always)]
// pub fn manhattan(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
//     x.iter().zip(y.iter()).map(|(&a, &b)| (a - b).abs()).sum()
// }

// #[inline(always)]
// pub fn cosine(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
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

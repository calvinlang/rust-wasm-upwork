mod helpers;

use helpers::{count_actual, count_expected, parse_float, Qual, Quant, Study, Test};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(serde::Serialize)]
pub struct Data {
    study_id: String,
    test_name: String,
    test_result_type: String,
    method_comparison: serde_json::Value,
    unmatched_samples: i64,
}

#[wasm_bindgen]
pub fn execute(studies: wasm_bindgen::JsValue, samples: wasm_bindgen::JsValue) -> JsValue {
    let studies_parsed: Vec<Study> = studies.into_serde().expect("failed to parse studies");

    let samples_parsed = samples
        .into_serde::<Vec<serde_json::Value>>()
        .expect("failed to parse samples");

    let results = merge(studies_parsed, samples_parsed);

    wasm_bindgen::JsValue::from_serde(&results).expect("failed to serialize results")
}

fn merge(studies: Vec<Study>, samples: Vec<serde_json::Value>) -> Vec<Data> {
    let samples_dict: HashMap<_, _> = samples
        .iter()
        .cloned()
        .map(|x| {
            (
                x.get("id")
                    .and_then(|id| id.as_str())
                    .expect("bad id")
                    .to_string(),
                x,
            )
        })
        .collect();

    studies
        .iter()
        .cloned()
        .filter_map(|study| combine_data(study, &samples_dict))
        .collect()
}

fn combine_data(study: Study, samples: &HashMap<String, serde_json::Value>) -> Option<Data> {
    let samples: Vec<serde_json::Value> = study
        .method_comparison
        .sample_ids
        .iter()
        .filter_map(|sample_id| samples.get(sample_id).and_then(|s| s.get(&study.id)))
        .map(|j| j.clone())
        .collect();

    let samples_count = study.method_comparison.sample_ids.len() as i64;

    // Successfully matched and parsed tests.
    let tests: Vec<Test> = samples
        .iter()
        .cloned()
        .filter_map(|x| serde_json::from_value(x).ok())
        .collect();

    let tests_count = tests.len() as i64;

    let method_comparison = match study.test_result_type.as_str() {
        "qual" => serde_json::to_value(Qual {
            samples_count: tests_count,
            pos_count: count_actual(&tests, "Pos") + count_expected(&tests, "Pos"),
            neg_count: count_actual(&tests, "Neg") + count_expected(&tests, "Neg"),
        })
        .ok()?,
        "quant" => serde_json::to_value(Quant {
            samples_count: tests_count,
            mean: if tests_count == 0 {
                0.0
            } else {
                tests
                    .iter()
                    .cloned()
                    .map(|x| vec![parse_float(x.actual), parse_float(x.expected)])
                    .flatten()
                    .sum::<f64>()
                    / (tests_count as f64)
            },
        })
        .ok()?,
        _ => serde_json::Value::Null,
    };

    if method_comparison == serde_json::Value::Null {
        println!("not calculated: {}", study.test_name);
        return None;
    }

    let study = Data {
        study_id: study.id,
        test_name: study.test_name,
        test_result_type: study.test_result_type,
        method_comparison,
        unmatched_samples: samples_count - tests_count,
    };

    Some(study)
}

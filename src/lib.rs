use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(serde::Deserialize, Clone)]
struct Study {
    id: String,
    #[serde(rename = "testName")]
    test_name: String,
    #[serde(rename = "testResultType")]
    test_result_type: String,
    #[serde(rename = "methodComparison")]
    method_comparison: Method,
}

#[derive(serde::Deserialize, Clone)]
struct Method {
    #[serde(rename = "sampleIds")]
    sample_ids: Vec<String>,
}

#[derive(serde::Serialize)]
struct Quant {
    samples_count: i32,
    mean: f32,
}

#[derive(serde::Serialize)]
struct Qual {
    samples_count: i32,
    pos_count: i32,
    neg_count: i32,
}

#[derive(serde::Deserialize, Clone)]
struct Test {
    actual: serde_json::Value,
    expected: serde_json::Value,
}

#[wasm_bindgen]
#[derive(serde::Serialize)]
pub struct Data {
    study_id: String,
    test_name: String,
    test_result_type: String,
    method_comparison: serde_json::Value,
}

fn parse2(study: Study, samples: &HashMap<String, serde_json::Value>) -> Option<Data> {
    let sms: Vec<serde_json::Value> = study
        .method_comparison
        .sample_ids
        .iter()
        .filter_map(|sample_id| samples.get(sample_id).and_then(|s| s.get(&study.id)))
        .map(|j| j.clone())
        .collect();

    let xs: Vec<Test> = sms
        .iter()
        .cloned()
        .filter_map(|x| serde_json::from_value(x).ok())
        .collect();

    let samples_count = xs.len() as i32;

    //let method_comparison = serde_json::Value::Null;
    let method_comparison = match study.test_result_type.as_str() {
        "qual" => serde_json::to_value(Qual {
            samples_count,
            pos_count: xs
                .iter()
                .filter(|x| x.actual == "Pos")
                .collect::<Vec<_>>()
                .len() as i32
                + xs.iter()
                    .filter(|x| x.expected == "Pos")
                    .collect::<Vec<_>>()
                    .len() as i32,
            neg_count: xs
                .iter()
                .filter(|x| x.actual == "Neg")
                .collect::<Vec<_>>()
                .len() as i32
                + xs.iter()
                    .filter(|x| x.expected == "Neg")
                    .collect::<Vec<_>>()
                    .len() as i32,
        })
        .ok()?,
        "quant" => serde_json::to_value(Quant {
            samples_count,
            mean: if samples_count == 0 {
                0.0
            } else {
                xs.iter()
                    .cloned()
                    .map(|x| {
                        vec![
                            //x.actual.parse::<f32>().unwrap_or(0.0),
                            //x.expected.parse::<f32>().unwrap_or(0.0),
                            1.1,
                        ]
                    })
                    .flatten()
                    .sum::<f32>()
                    / (samples_count as f32)
            },
        })
        .ok()?,
        //_ => serde_json::Value::String(test_result_type.clone()),
        _ => serde_json::Value::Null,
    };

    let study = Data {
        study_id: study.id,
        test_name: study.test_name,
        test_result_type: study.test_result_type,
        method_comparison,
    };

    return Some(study);
}

fn parse(study: serde_json::Value, samples: &HashMap<String, serde_json::Value>) -> Option<Data> {
    let study_id = study.get("id")?.as_str()?.to_string();

    let test_result_type = study.get("testResultType")?.as_str()?;

    let sample_ids = study
        .get("methodComparison")?
        .as_object()?
        .get("sampleIds")?
        .as_array()?
        .iter()
        .filter_map(|x| x.as_str().map(|y| y.to_string()))
        .collect::<Vec<_>>();

    let sms: Vec<serde_json::Value> = sample_ids
        .iter()
        .filter_map(|sample_id| samples.get(sample_id).and_then(|s| s.get(&study_id)))
        .map(|j| j.clone())
        .collect();

    let xs: Vec<Test> = sms
        .iter()
        .cloned()
        .filter_map(|x| serde_json::from_value(x).ok())
        .collect();

    let samples_count = xs.len() as i32;

    //let method_comparison = serde_json::Value::Null;
    let method_comparison = match test_result_type {
        "qual" => serde_json::to_value(Qual {
            samples_count,
            pos_count: xs
                .iter()
                .filter(|x| x.actual == "Pos")
                .collect::<Vec<_>>()
                .len() as i32
                + xs.iter()
                    .filter(|x| x.expected == "Pos")
                    .collect::<Vec<_>>()
                    .len() as i32,
            neg_count: xs
                .iter()
                .filter(|x| x.actual == "Neg")
                .collect::<Vec<_>>()
                .len() as i32
                + xs.iter()
                    .filter(|x| x.expected == "Neg")
                    .collect::<Vec<_>>()
                    .len() as i32,
        })
        .ok()?,
        "quant" => serde_json::to_value(Quant {
            samples_count,
            mean: if samples_count == 0 {
                0.0
            } else {
                xs.iter()
                    .cloned()
                    .map(|x| {
                        vec![
                            //x.actual.parse::<f32>().unwrap_or(0.0),
                            //x.expected.parse::<f32>().unwrap_or(0.0),
                            1.1,
                        ]
                    })
                    .flatten()
                    .sum::<f32>()
                    / (samples_count as f32)
            },
        })
        .ok()?,
        //_ => serde_json::Value::String(test_result_type.clone()),
        _ => serde_json::Value::Null,
    };

    let study = Data {
        study_id,
        test_name: study.get("testName")?.as_str()?.to_string(),
        test_result_type: test_result_type.to_string(),
        method_comparison,
    };

    return Some(study);
}

fn handle(studies: Vec<Study>, samples: Vec<serde_json::Value>) -> Vec<Data> {
    let samps: HashMap<_, _> = samples
        .iter()
        .cloned()
        .map(|x| {
            (
                x.get("id").expect("err1").as_str().expect("!").to_string(),
                x,
            )
        })
        .collect();

    studies
        .iter()
        .cloned()
        .filter_map(|st| parse2(st, &samps))
        .collect()
}

#[wasm_bindgen]
pub fn obj(studies: wasm_bindgen::JsValue, samples: wasm_bindgen::JsValue) -> JsValue {
    let sts: Vec<Study> = studies.into_serde().expect("studies decode");

    let samps = samples
        .into_serde::<Vec<serde_json::Value>>()
        .expect("hash");

    let xs = handle(sts, samps);

    wasm_bindgen::JsValue::from_serde(&xs).expect("exit")
}

fn main() {
    let studies: Vec<Study> = serde_json::from_reader(std::io::BufReader::new(
        File::open("./src/datasets/studies.json").unwrap(),
    ))
    .unwrap();

    let samples: Vec<serde_json::Value> = serde_json::from_reader(std::io::BufReader::new(
        File::open("./src/datasets/samples.json").unwrap(),
    ))
    .unwrap();

    let out = handle(studies, samples);

    File::create("./out.json")
        .unwrap()
        .write_all(serde_json::to_string(&out).unwrap().as_bytes())
        .unwrap();

    ()
}

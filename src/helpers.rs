pub fn count_actual(tests: &Vec<Test>, tag: &str) -> i64 {
    tests
        .iter()
        .filter(|x| x.actual == tag)
        .collect::<Vec<_>>()
        .len() as i64
}

pub fn count_expected(tests: &Vec<Test>, tag: &str) -> i64 {
    tests
        .iter()
        .filter(|x| x.expected == tag)
        .collect::<Vec<_>>()
        .len() as i64
}

pub fn parse_float(val: serde_json::Value) -> f64 {
    val.as_f64().unwrap_or(
        val.as_str()
            .and_then(|x| x.parse::<f64>().ok())
            .unwrap_or(val.as_i64().map(|x| x as f64).unwrap_or(0.0)),
    )
}

#[derive(serde::Serialize)]
pub struct Quant {
    pub samples_count: i64,
    pub mean: f64,
}

#[derive(serde::Serialize)]
pub struct Qual {
    pub samples_count: i64,
    pub pos_count: i64,
    pub neg_count: i64,
}

#[derive(serde::Deserialize, Clone)]
pub struct Study {
    pub id: String,
    #[serde(rename = "testName")]
    pub test_name: String,
    #[serde(rename = "testResultType")]
    pub test_result_type: String,
    #[serde(rename = "methodComparison")]
    pub method_comparison: Method,
}

#[derive(serde::Deserialize, Clone)]
pub struct Method {
    #[serde(rename = "sampleIds")]
    pub sample_ids: Vec<String>,
}

#[derive(serde::Deserialize, Clone)]
pub struct Test {
    pub actual: serde_json::Value,
    pub expected: serde_json::Value,
}

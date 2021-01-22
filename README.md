# Rust Wasm Project

### To build the WebAssembly
`wasm-pack build --target web -d public/pkg`

[wasm-pack](https://rustwasm.github.io/wasm-pack/)

### To watch and make changes
`cargo watch`

---

In `/src/datasets` there are two data sets.

studies.js - is an array of study objects.

### Important Fields

`testName`
`studyId` is a unique identifier
`testResultType` can be "quant" "qual" or "other"
`methodComparison` is an object with a `sampleIds` array of unique strings for samples

samples.js - is an array of samples objects

### Important Fields

`sampleId` unique identifier of the sample
`[studyId]` from the studies object. This sometimes contains an `actual` and/or `expected` values that can be strings or integers.

## Objective

From the react front end, create a button that will start a Rust wasm function that can accept the `studies` and `samples` array.

For each `study` in the studies array, in `methodComparison` there is the array of `sampleIds`.

For each of the samples with a matching `sampleId`, in the samples' field `[studyId]` there are `actual` and `expected` fields.

If the `study.testResultType` is `"quant"`, return the average value (mean) of all the `actual` field values and the expected field values. If it is a string you must parse to a float.

If the `study.testResultType` is `"qual"`, return the count of "Pos" and count of "Neg"

These should return an array of studies like so

```
[
  {
    studyId: "string",
    testName: "string",
    testResultType: "string",
    methodComparison: {
      samplesCount: int,
      mean: number,     // if testResultType === "quant"
      posCount: int,    // if testResultType === "qual",
      negCount: int     // if testResultType === "qual"
    }
  }, ...
]


```

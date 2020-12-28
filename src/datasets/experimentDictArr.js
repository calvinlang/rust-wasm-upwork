const experimentDictArr = [
  {
    experimentKey: "trueness",
    name: "Trueness",
    abbrev: "Trns",
    url: "trueness",
    useHeaderKeys: true,
    quant: [
      "label",
      "source",
      "actual",
      "expected",
      "useEQARangeQuant",
      "truenessPassQuant",
    ],
    qual: ["label", "source", "actual", "expected", "agree"],
    other: ["label", "source", "actual", "expected", "agree"],
    accCrit: {
      minSamples: {
        title: "Min Samples",
        titleExtended: "Min Samples (5-100)",
        default: 20,

        min: 5,
        max: 100,
        noDecimals: true,
      },
      minPercentWithinError: {
        title: "Min Pass Rate",
        titleExtended: "Sample Pass Rate (%)",
        default: 95,
        min: 50,
        max: 100,
        noDecimals: false,
        percent: true,
      },
    },
  },
  {
    experimentKey: "precision",
    name: "Precision",
    abbrev: "Prec",
    url: "precision",
    runsBased: true,
    quant: true,
    other: true,
    qual: true,
    accCrit: {
      minSamples: {
        title: "Min Samples",
        titleExtended: "Min Samples (5-100)",
        default: 20,
        min: 5,
        max: 100,
        noDecimals: true,
      },
      minDaysPerSample: {
        title: "Min Days per Sample/Level",
        titleExtended: "Min Days Per Sample (1-30)",
        default: 1,
        min: 1,
        max: 30,
        noDecimals: true,
      },
      minRunsPerSample: {
        title: "Min Runs per Level",
        titleExtended: "Min Runs per Level (1-30)",
        default: 1,
        min: 1,
        max: 30,
        noDecimals: true,
      },
      minMeasurementsPerRun: {
        title: "Min Measurements/Run",
        titleExtended: "Min Measurements per (5-100)",
        default: 1,
        min: 1,
        max: 30,
        noDecimals: true,
      },
      runPassPercent: {
        title: "Within-Run Pass Rate",
        titleExtended: "Within-Run Pass Rate (%)",
        default: 95,
        min: 50,
        max: 100,
        noDecimals: true,
        percent: true,
      },
      withinDayPassPercent: {
        title: "Within-Day Pass Rate",
        titleExtended: "Within-Day Pass Rate (%)",
        default: 95,
        min: 50,
        max: 100,
        noDecimals: true,
        percent: true,
      },
      betweenDayPassPercent: {
        title: "Between-Day Pass Rate",
        titleExtended: "Between-Day Pass Rate (%)",
        default: 95,
        min: 50,
        max: 100,
        noDecimals: true,
        percent: true,
      },
    },
  },
  {
    experimentKey: "methodComparison",
    name: "Method Comparison",
    abbrev: "MC",
    url: "method-comparison",
    useHeaderKeys: true,
    quant: ["label", "actual", "expected", "bias"],
    qual: ["label", "actual", "expected", "agree"],
    other: ["label", "actual", "expected", "agree"],
    accCrit: {
      minSamples: {
        title: "Min Samples",
        titleExtended: "Min Samples (5-100)",
        default: 20,

        min: 5,
        max: 100,
        noDecimals: true,
      },
      minPercentWithinError: {
        title: "Min Pass Rate",
        titleExtended: "Sample Pass Rate (%)",
        default: 95,
        min: 50,
        max: 100,
        noDecimals: false,
        percent: true,
      },
    },
  },
  {
    experimentKey: "linearity",
    name: "Linearity",
    quant: true, // true
    other: false, // true
    qual: false, // false
    abbrev: "Lin",
    url: "linearity",
    runsBased: true,
  },
  {
    experimentKey: "limitOfBlank",
    name: "Limit of Blank",
    abbrev: "LoB",
    url: "limit-of-blank",
    quant: false, // true
    other: false, // true
    qual: false,
    runsBased: true,
  },
  {
    experimentKey: "limitOfDetection",
    name: "Limit of Detection",
    abbrev: "LoD",
    url: "limit-of-detection",
    quant: false, // true
    other: false, // true
    qual: false,
    runsBased: true,
  },
  {
    experimentKey: "limitOfQuantitation",
    name: "Limit of Quanitiation",
    abbrev: "LoQ",
    url: "limit-of-quantitation",
    quant: false, // true
    other: false, // true
    qual: false,
    runsBased: true,
  },
  {
    experimentKey: "carryOver",
    name: "Carry Over",
    abbrev: "CO",
    url: "carry-over",
    quant: false, // true
    other: false, // true
    qual: false,
    runsBased: true,
  },
  {
    experimentKey: "interference",
    name: "Interference",
    abbrev: "Int",
    url: "interference",
    quant: false, // true
    other: false, // true
    qual: false, // true
  },
  {
    experimentKey: "hookEffect",
    name: "Hook Effect",
    abbrev: "HE",
    url: "hook-effect",
    quant: false, // true
    other: false, // true
    qual: false, // true
  },
  {
    experimentKey: "autoDilution",
    name: "Auto Dilution",
    abbrev: "fill",
    url: "auto-dilution",
    quant: false, // true
    other: false, // true
    qual: false,
  },
];

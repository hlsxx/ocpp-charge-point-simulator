use chrono::Utc;
use rand::seq::IndexedRandom;
use rust_ocpp::v1_6::types::{Location, Measurand, MeterValue, Phase, ReadingContext, SampledValue, UnitOfMeasure, ValueFormat};
use rand::Rng;

pub trait MockData {
  fn mock_data() -> Self;
}

impl MockData for SampledValue {
  fn mock_data() -> Self {
    let mut rng = rand::rng();

    let context_options = [
      ReadingContext::InterruptionBegin,
      ReadingContext::SamplePeriodic,
      ReadingContext::TransactionBegin,
    ];

    let format_options = [ValueFormat::Raw];

    let location_options = [Location::Outlet, Location::Body];

    let measurand_options = [
      Measurand::CurrentImport,
      Measurand::EnergyActiveImportRegister,
      Measurand::PowerActiveImport,
      Measurand::Voltage,
    ];

    let phase_options = [
      Some(Phase::L1),
      Some(Phase::L2),
      Some(Phase::L3),
      Some(Phase::L1N),
      Some(Phase::L2N),
      Some(Phase::L3N),
      None,
    ];

    let unit_options = [
      UnitOfMeasure::A,
      UnitOfMeasure::Kw,
      UnitOfMeasure::Wh,
      UnitOfMeasure::V,
    ];

    SampledValue {
      context: Some(context_options.choose(&mut rng).unwrap().clone()),
      format: Some(format_options.choose(&mut rng).unwrap().clone()),
      location: Some(location_options.choose(&mut rng).unwrap().clone()),
      measurand: Some(measurand_options.choose(&mut rng).unwrap().clone()),
      phase: phase_options.choose(&mut rng).unwrap().as_ref().map(|p| p.clone()),
      unit: Some(unit_options.choose(&mut rng).unwrap().clone()),
      value: format!("{:.3}", rng.random_range(0.0..1000.0)),
    }
  }
}

impl MockData for MeterValue {
  fn mock_data() -> Self {
    let mut rng = rand::rng();
    let count = rng.random_range(5..=15);

    MeterValue {
      timestamp: Utc::now(),
      sampled_value: (0..count).map(|_| SampledValue::mock_data()).collect(),
    }
  }
}

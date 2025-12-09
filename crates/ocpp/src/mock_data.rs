use chrono::Utc;
use rand::Rng;
use rand::seq::IndexedRandom;
use rust_ocpp::v1_6::types::{
  Location, Measurand, MeterValue, Phase, ReadingContext, SampledValue, UnitOfMeasure, ValueFormat,
};

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
      phase: phase_options.choose(&mut rng).unwrap().clone(),
      unit: Some(unit_options.choose(&mut rng).unwrap().clone()),
      value: format!("{:.3}", rng.random_range(0.0..1000.0)),
    }
  }
}

impl MockData for MeterValue {
  fn mock_data() -> Self {
    let mut rng = rand::rng();
    let mut sampled_values = Vec::new();

    // Realistic voltage ranges for each phase (in Volts)
    let voltage_l1 = rng.random_range(220.0..240.0);
    let voltage_l2 = rng.random_range(220.0..240.0);
    let voltage_l3 = rng.random_range(220.0..240.0);

    // Realistic current ranges for each phase (in Amps)
    let current_l1 = rng.random_range(5.0..32.0);
    let current_l2 = rng.random_range(5.0..32.0);
    let current_l3 = rng.random_range(5.0..32.0);

    // Calculate power for each phase (P = V * I) in kW
    let power_l1 = (voltage_l1 * current_l1) / 1000.0;
    let power_l2 = (voltage_l2 * current_l2) / 1000.0;
    let power_l3 = (voltage_l3 * current_l3) / 1000.0;

    // Energy register (cumulative, in Wh)
    let energy_register = rng.random_range(1000.0..50000.0);

    // All readings use consistent context, format, and location
    let context = ReadingContext::InterruptionBegin;
    let format = ValueFormat::Raw;
    let location = Location::Outlet; // Only Outlet

    // Current readings for each phase (L1, L2, L3)
    sampled_values.push(SampledValue {
      context: Some(context.clone()),
      format: Some(format.clone()),
      location: Some(location.clone()),
      measurand: Some(Measurand::CurrentImport),
      phase: Some(Phase::L1),
      unit: Some(UnitOfMeasure::A),
      value: format!("{:.3}", current_l1),
    });

    sampled_values.push(SampledValue {
      context: Some(context.clone()),
      format: Some(format.clone()),
      location: Some(location.clone()),
      measurand: Some(Measurand::CurrentImport),
      phase: Some(Phase::L2),
      unit: Some(UnitOfMeasure::A),
      value: format!("{:.3}", current_l2),
    });

    sampled_values.push(SampledValue {
      context: Some(context.clone()),
      format: Some(format.clone()),
      location: Some(location.clone()),
      measurand: Some(Measurand::CurrentImport),
      phase: Some(Phase::L3),
      unit: Some(UnitOfMeasure::A),
      value: format!("{:.3}", current_l3),
    });

    // Energy register (cumulative, no phase)
    sampled_values.push(SampledValue {
      context: Some(context.clone()),
      format: Some(format.clone()),
      location: Some(location.clone()),
      measurand: Some(Measurand::EnergyActiveImportRegister),
      phase: None,
      unit: Some(UnitOfMeasure::Wh),
      value: format!("{:.3}", energy_register),
    });

    // Power readings for each phase (L1-N, L2-N, L3-N)
    sampled_values.push(SampledValue {
      context: Some(context.clone()),
      format: Some(format.clone()),
      location: Some(location.clone()),
      measurand: Some(Measurand::PowerActiveImport),
      phase: Some(Phase::L1N),
      unit: Some(UnitOfMeasure::Kw),
      value: format!("{:.3}", power_l1),
    });

    sampled_values.push(SampledValue {
      context: Some(context.clone()),
      format: Some(format.clone()),
      location: Some(location.clone()),
      measurand: Some(Measurand::PowerActiveImport),
      phase: Some(Phase::L2N),
      unit: Some(UnitOfMeasure::Kw),
      value: format!("{:.3}", power_l2),
    });

    sampled_values.push(SampledValue {
      context: Some(context.clone()),
      format: Some(format.clone()),
      location: Some(location.clone()),
      measurand: Some(Measurand::PowerActiveImport),
      phase: Some(Phase::L3N),
      unit: Some(UnitOfMeasure::Kw),
      value: format!("{:.3}", power_l3),
    });

    // Voltage readings for each phase (L1-N, L2-N, L3-N)
    sampled_values.push(SampledValue {
      context: Some(context.clone()),
      format: Some(format.clone()),
      location: Some(location.clone()),
      measurand: Some(Measurand::Voltage),
      phase: Some(Phase::L1N),
      unit: Some(UnitOfMeasure::V),
      value: format!("{:.3}", voltage_l1),
    });

    sampled_values.push(SampledValue {
      context: Some(context.clone()),
      format: Some(format.clone()),
      location: Some(location.clone()),
      measurand: Some(Measurand::Voltage),
      phase: Some(Phase::L2N),
      unit: Some(UnitOfMeasure::V),
      value: format!("{:.3}", voltage_l2),
    });

    sampled_values.push(SampledValue {
      context: Some(context.clone()),
      format: Some(format.clone()),
      location: Some(location.clone()),
      measurand: Some(Measurand::Voltage),
      phase: Some(Phase::L3N),
      unit: Some(UnitOfMeasure::V),
      value: format!("{:.3}", voltage_l3),
    });

    MeterValue {
      timestamp: Utc::now(),
      sampled_value: sampled_values,
    }
  }
}

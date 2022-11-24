use crate::prelude::*;

#[derive(Debug)]
pub enum Command {
    ReadInputs(config::Inverter, i16),
    ReadInput(config::Inverter, i16, i16),
    ReadHold(config::Inverter, i16, i16),
    ReadParam(config::Inverter, i16),
    SetHold(config::Inverter, i16, i16),
    ChargeRate(config::Inverter, i16),
    DischargeRate(config::Inverter, i16),
    AcCharge(config::Inverter, bool),
    ForcedDischarge(config::Inverter, bool),
    AcChargeRate(config::Inverter, i16),
    AcChargeSocLimit(config::Inverter, i16),
    DischargeCutoffSocLimit(config::Inverter, i16),
}

impl Command {
    pub fn to_result_topic(&self) -> String {
        use Command::*;

        let rest = match self {
            ReadInputs(inverter, c) => format!("{}/read/inputs/{}", inverter.datalog(), c),
            ReadInput(inverter, register, _) => {
                format!("{}/read/input/{}", inverter.datalog(), register)
            }
            ReadHold(inverter, register, _) => {
                format!("{}/read/hold/{}", inverter.datalog(), register)
            }
            ReadParam(inverter, register) => {
                format!("{}/read/param/{}", inverter.datalog(), register)
            }
            SetHold(inverter, register, _) => {
                format!("{}/set/hold/{}", inverter.datalog(), register)
            }
            AcCharge(inverter, _) => format!("{}/set/ac_charge", inverter.datalog()),
            ForcedDischarge(inverter, _) => format!("{}/set/forced_discharge", inverter.datalog()),
            ChargeRate(inverter, _) => format!("{}/set/charge_rate_pct", inverter.datalog()),
            DischargeRate(inverter, _) => format!("{}/set/discharge_rate_pct", inverter.datalog()),
            AcChargeRate(inverter, _) => format!("{}/set/ac_charge_rate_pct", inverter.datalog()),
            AcChargeSocLimit(inverter, _) => {
                format!("{}/set/ac_charge_soc_limit_pct", inverter.datalog())
            }
            DischargeCutoffSocLimit(inverter, _) => {
                format!("{}/set/discharge_cutoff_soc_limit_pct", inverter.datalog())
            }
        };

        format!("result/{}", rest)
    }
}

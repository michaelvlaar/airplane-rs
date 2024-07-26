const AVGAS_FUEL_DENSITY_KG_LITER: f64 = 0.72;
const MOGAS_FUEL_DENSITY_KG_LITER: f64 = 0.74;

const LITERS_IN_GALLON: f64 = 378541.0 / 100000.0;

pub enum LeverArm {
    Meter(f64),
}

impl LeverArm {
    pub fn meter(&self) -> f64 {
        match self {
            LeverArm::Meter(m) => *m,
        }
    }
}

pub enum Volume {
    Liter(f64),
    Gallon(f64),
}

pub enum Mass {
    Kilo(f64),
    Avgas(Volume),
    Mogas(Volume),
}

impl Mass {
    pub fn kilo(&self) -> f64 {
        match self {
            Mass::Kilo(kg) => *kg,
            Mass::Avgas(l) => match l {
                Volume::Liter(l) => l * AVGAS_FUEL_DENSITY_KG_LITER,
                Volume::Gallon(g) => g * LITERS_IN_GALLON * AVGAS_FUEL_DENSITY_KG_LITER,
            },
            Mass::Mogas(l) => match l {
                Volume::Liter(l) => l * MOGAS_FUEL_DENSITY_KG_LITER,
                Volume::Gallon(g) => g * LITERS_IN_GALLON * MOGAS_FUEL_DENSITY_KG_LITER,
            },
        }
    }
}

pub struct Moment {
    lever_arm: LeverArm,
    mass: Mass,
}

impl Moment {
    pub fn new(lever_arm: LeverArm, mass: Mass) -> Moment {
        Moment { lever_arm, mass }
    }

    pub fn lever_arm(&self) -> &LeverArm {
        &self.lever_arm
    }

    pub fn mass(&self) -> &Mass {
        &self.mass
    }

    pub fn total(&self) -> MassMoment {
        MassMoment::KgM(self.mass.kilo() * self.lever_arm.meter())
    }
}

pub enum MassMoment {
    KgM(f64),
}

impl MassMoment {
    pub fn kgm(&self) -> f64 {
        match self {
            MassMoment::KgM(kgm) => *kgm,
        }
    }
}

/// Positive numbers represent reference aft of datum.
pub enum CenterOfGravity {
    Meter(f64),
    Millimeter(f64),
}

impl CenterOfGravity {
    pub fn meter(&self) -> f64 {
        match self {
            CenterOfGravity::Meter(m) => *m,
            CenterOfGravity::Millimeter(mm) => mm / 1000.0,
        }
    }
}

pub struct Limits {
    minimum_weight: Mass,
    mtow: Mass,
    forward_cg_limit: CenterOfGravity,
    rearward_cg_limit: CenterOfGravity,
}

impl Limits {
    pub fn new(
        minimum_weight: Mass,
        mtow: Mass,
        forward_cg_limit: CenterOfGravity,
        rearward_cg_limit: CenterOfGravity,
    ) -> Limits {
        Limits {
            minimum_weight,
            mtow,
            forward_cg_limit,
            rearward_cg_limit,
        }
    }

    pub fn minimum_weight(&self) -> &Mass {
        &self.minimum_weight
    }

    pub fn mtow(&self) -> &Mass {
        &self.mtow
    }

    pub fn forward_cg_limit(&self) -> &CenterOfGravity {
        &self.forward_cg_limit
    }

    pub fn rearward_cg_limit(&self) -> &CenterOfGravity {
        &self.rearward_cg_limit
    }
}

pub struct Airplane {
    callsign: String,
    moments: Vec<Moment>,
    limits: Limits,
}

impl Airplane {
    pub fn new(callsign: String, moments: Vec<Moment>, limits: Limits) -> Airplane {
        Airplane {
            callsign,
            moments,
            limits,
        }
    }

    pub fn limits(&self) -> &Limits {
        &self.limits
    }

    fn center_of_gravity(&self) -> CenterOfGravity {
        let kg_mass = self.total_mass().kilo();
        let kgm_moment = self.total_mass_moment().kgm();

        CenterOfGravity::Meter(kgm_moment / kg_mass)
    }

    pub fn total_mass_moment(&self) -> MassMoment {
        MassMoment::KgM(self.moments.iter().map(|m| m.total().kgm()).sum())
    }

    pub fn total_mass(&self) -> Mass {
        Mass::Kilo(self.moments.iter().map(|m| m.mass.kilo()).sum())
    }

    pub fn within_limits(&self) -> bool {
        let cg = self.center_of_gravity().meter();
        self.total_mass().kilo() <= self.limits.mtow.kilo()
            && cg <= self.limits.rearward_cg_limit.meter()
            && cg >= self.limits.forward_cg_limit.meter()
    }

    pub fn callsign(&self) -> &String {
        &self.callsign
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn airplane(within_limits: bool) -> Airplane {
        let pilot_mass = if within_limits {
            Mass::Kilo(80.0)
        } else {
            Mass::Kilo(95.0)
        };

        Airplane::new(
            String::from("PHDHA"),
            vec![
                Moment::new(LeverArm::Meter(0.4294), Mass::Kilo(517.0)),
                Moment::new(LeverArm::Meter(0.515), pilot_mass),
                Moment::new(LeverArm::Meter(0.515), Mass::Kilo(89.0)),
                Moment::new(LeverArm::Meter(1.3), Mass::Kilo(5.0)),
                Moment::new(LeverArm::Meter(0.325), Mass::Avgas(Volume::Liter(62.0))),
            ],
            Limits::new(
                Mass::Kilo(558.0),
                Mass::Kilo(750.0),
                CenterOfGravity::Millimeter(427.0),
                CenterOfGravity::Millimeter(523.0),
            ),
        )
    }

    #[test]
    fn calculate_kg_moment() {
        let m = Moment::new(LeverArm::Meter(0.4294), Mass::Kilo(517.0));
        let MassMoment::KgM(kgm) = m.total();

        assert_eq!(517.0 * 0.4294, kgm);
    }

    #[test]
    fn calculate_cg() {
        assert_eq!(
            (((0.4294 * 517.0)
                + (0.515 * 80.0)
                + (0.515 * 89.0)
                + (1.3 * 5.0)
                + (0.325 * AVGAS_FUEL_DENSITY_KG_LITER * 62.0))
                / (517.0 + 80.0 + 89.0 + 5.0 + (62.0 * AVGAS_FUEL_DENSITY_KG_LITER))),
            airplane(true).center_of_gravity().meter()
        );
    }

    #[test]
    fn outside_of_limits() {
        assert!(!airplane(false).within_limits());
    }

    #[test]
    fn inside_of_limits() {
        assert!(airplane(true).within_limits());
    }
}

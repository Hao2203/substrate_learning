pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait Duration {
    fn duration(&self) -> u32;
}

impl Duration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 55,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 50,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration() {
        assert_eq!(TrafficLight::Red.duration(), 55);
        assert_eq!(TrafficLight::Yellow.duration(), 5);
        assert_eq!(TrafficLight::Green.duration(), 50);
    }
}

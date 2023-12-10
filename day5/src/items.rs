pub trait UncannyItem: Copy {
    fn identifier() -> &'static str;
    fn from_int(value: usize) -> Self;
    fn value(&self) -> usize;
}

#[derive(Debug, Clone, Copy)]
pub struct Seed(usize);
impl UncannyItem for Seed {
    fn identifier() -> &'static str {
        "seed"
    }

    fn from_int(value: usize) -> Self {
        Self(value)
    }

    fn value(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Soil(usize);
impl UncannyItem for Soil {
    fn identifier() -> &'static str {
        "soil"
    }

    fn from_int(value: usize) -> Self {
        Self(value)
    }

    fn value(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Fertilizer(usize);
impl UncannyItem for Fertilizer {
    fn identifier() -> &'static str {
        "fertilizer"
    }

    fn from_int(value: usize) -> Self {
        Self(value)
    }

    fn value(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Water(usize);
impl UncannyItem for Water {
    fn identifier() -> &'static str {
        "water"
    }

    fn from_int(value: usize) -> Self {
        Self(value)
    }

    fn value(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Light(usize);
impl UncannyItem for Light {
    fn identifier() -> &'static str {
        "light"
    }

    fn from_int(value: usize) -> Self {
        Self(value)
    }

    fn value(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Temperature(usize);
impl UncannyItem for Temperature {
    fn identifier() -> &'static str {
        "temperature"
    }

    fn from_int(value: usize) -> Self {
        Self(value)
    }

    fn value(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Humidity(usize);
impl UncannyItem for Humidity {
    fn identifier() -> &'static str {
        "humidity"
    }

    fn from_int(value: usize) -> Self {
        Self(value)
    }

    fn value(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Location(usize);
impl UncannyItem for Location {
    fn identifier() -> &'static str {
        "location"
    }

    fn from_int(value: usize) -> Self {
        Self(value)
    }

    fn value(&self) -> usize {
        self.0
    }
}

use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq)]
pub enum DnaBase {
    A,
    C,
    G,
    T,
}

impl TryFrom<char> for DnaBase {
    type Error = char;
    fn try_from(dna: char) -> Result<Self, Self::Error> {
        match dna {
            'A' => Ok(Self::A),
            'C' => Ok(Self::C),
            'G' => Ok(Self::G),
            'T' => Ok(Self::T),
            c => Err(c),
        }
    }
}

impl From<RnaBase> for DnaBase {
    fn from(rna: RnaBase) -> Self {
        match rna {
            RnaBase::C => DnaBase::G,
            RnaBase::G => DnaBase::C,
            RnaBase::A => DnaBase::T,
            RnaBase::U => DnaBase::A,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dna(Vec<DnaBase>);

#[derive(Debug, PartialEq, Eq)]
pub enum RnaBase {
    A,
    C,
    G,
    U,
}

impl TryFrom<char> for RnaBase {
    type Error = char;
    fn try_from(rna: char) -> Result<Self, Self::Error> {
        match rna {
            'A' => Ok(Self::A),
            'C' => Ok(Self::C),
            'G' => Ok(Self::G),
            'U' => Ok(Self::U),
            c => Err(c),
        }
    }
}

impl From<DnaBase> for RnaBase {
    fn from(dna: DnaBase) -> Self {
        match dna {
            DnaBase::G => RnaBase::C,
            DnaBase::C => RnaBase::G,
            DnaBase::T => RnaBase::A,
            DnaBase::A => RnaBase::U,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(Vec<RnaBase>);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let dna = dna
            .chars()
            .enumerate()
            .map(|(i, c)| DnaBase::try_from(c).map_err(|_| i))
            .collect::<Result<_, usize>>()?;
        Ok(Dna(dna))
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .into_iter()
            .map(|d| RnaBase::from(d))
            .collect::<Vec<RnaBase>>())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let rna = rna
            .chars()
            .enumerate()
            .map(|(i, c)| RnaBase::try_from(c).map_err(|_| i))
            .collect::<Result<_, usize>>()?;
        Ok(Rna(rna))
    }
}

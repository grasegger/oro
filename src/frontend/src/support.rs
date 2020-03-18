use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Contract {
    pub variant: SupportPackageVariant,
    pub addons: Vec<SupportPackage>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupportPackage { 
    pub pkgname: String,
    pub cms: String,
    pub hours: i16,
    pub startQuarter: i16,
    pub stopQuarter: i16,
    pub yearly: bool
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupportPackageVariant { 
    pub pkgname: String,
    pub cms: String,
    pub hours: i16,
    pub startQuarter: i16,
    pub stopQuarter: i16,
    pub yearly: bool,
    pub addCarry: f64,
    pub title: String
}

use std::collections::{HashMap, LinkedList};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct PackagistPackageDto {
    pub version: String,
}

#[derive(Deserialize)]
pub struct PackagistPackageResponseEnvelope {
    pub packages: HashMap<String, LinkedList<PackagistPackageDto>>,
}

// Copyright (c) 2022 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//
use anyhow::{Context, Result};
use key_value_storage::StorageBackendConfig;
use serde::Deserialize;

use crate::extractors::ExtractorsConfig;

#[derive(Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Config {
    #[serde(default)]
    pub storage: StorageBackendConfig,

    #[serde(default)]
    pub extractors: Option<ExtractorsConfig>,
}

impl Config {
    pub fn from_file(config_path: &str) -> Result<Self> {
        let c = config::Config::builder()
            .add_source(config::File::with_name(config_path))
            .build()?;

        let res = c.try_deserialize().context("invalid config")?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use key_value_storage::KeyValueStorageType;

    #[test]
    fn docker_compose_default_rvps_config_is_correct() {
        let config_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../kbs/config/docker-compose/rvps.json");
        let config = Config::from_file(config_path.to_str().unwrap())
            .expect("Docker Compose RVPS config should be valid");

        assert_eq!(config.storage.storage_type, KeyValueStorageType::LocalFs);
        assert_eq!(
            config
                .storage
                .backends
                .local_fs
                .as_ref()
                .expect("LocalFs backend should be configured")
                .dir_path,
            "/opt/confidential-containers/attestation-service/reference_values"
        );
    }
}

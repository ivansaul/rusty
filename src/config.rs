use serde::Deserialize;

pub trait Validate {
    fn validate(&self) -> eyre::Result<()>;
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub general: General,
}

impl Validate for Config {
    fn validate(&self) -> eyre::Result<()> {
        self.general.validate()?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct General {
    pub log_level: String,
}

impl Default for General {
    fn default() -> Self {
        Self {
            log_level: "info".to_string(),
        }
    }
}

impl Validate for General {
    fn validate(&self) -> eyre::Result<()> {
        let mut validation_errors: Vec<eyre::Report> = vec![];

        if !["trace", "debug", "info", "warn", "error"].contains(&self.log_level.as_str()) {
            validation_errors.push(eyre::eyre!(
                "log_level must be one of trace, debug, info, warn, error"
            ));
        }

        if !validation_errors.is_empty() {
            return Err(eyre::eyre!("Validation errors: {:?}", validation_errors));
        }

        Ok(())
    }
}

impl Config {
    pub fn redact(&self) -> Self {
        todo!()
    }
}

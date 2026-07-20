use semver::Version;
use serde::{Deserialize, Serialize};

pub mod archipelago;

#[derive(Serialize, Deserialize)]
pub struct ServerVersions {
    /// Current version of the launcher.
    pub launcher: Version,
    /// Current version of archipelago. Isn't kept in sync with the github repo as there might be some breaking changes we need to deal with.
    pub archipelago: Version,
}

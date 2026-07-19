pub enum DefaultCards {
    InstallApWorld,
    SlotManager,
}
impl TryFrom<String> for DefaultCards {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Install ApWorld" => Ok(DefaultCards::InstallApWorld),
            "Slot Manager" => Ok(DefaultCards::SlotManager),
            _ => Err(value),
        }
    }
}
impl From<DefaultCards> for String {
    fn from(value: DefaultCards) -> Self {
        match value {
            DefaultCards::InstallApWorld => "Install ApWorld",
            DefaultCards::SlotManager => "Slot Manager",
        }
        .to_string()
    }
}

use gpui::{InvalidKeystrokeError, Keystroke};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Bind {
    pub exit: bool,
    pub bind: Keystroke,
    pub callback: String,
}
impl Bind {
    pub fn matches(&self, stroke: &Keystroke) -> bool {
        &self.bind == stroke
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BindSerde {
    bind: String,
    callback: String,
    pub exit: bool,
}

impl TryFrom<BindSerde> for Bind {
    type Error = InvalidKeystrokeError;
    fn try_from(value: BindSerde) -> Result<Self, Self::Error> {
        Ok(Bind {
            bind: Keystroke::parse(&value.bind)?,
            callback: value.callback,
            exit: value.exit,
        })
    }
}
impl TryFrom<&BindSerde> for Bind {
    type Error = InvalidKeystrokeError;
    fn try_from(value: &BindSerde) -> Result<Self, Self::Error> {
        Ok(Bind {
            bind: Keystroke::parse(&value.bind)?,
            callback: value.callback.clone(),
            exit: value.exit,
        })
    }
}

use gym_rs::{ActionType};

pub struct Action {
    continuous: Option<Vec<f64>>,
    discrete: Option<u8>,
}

impl Action {
    pub fn from_continuous(continuous: Vec<f64>) -> Self {
        Self {
            continuous: Some(continuous),
            discrete: None,
        }
    }

    pub fn from_discrete(discrete: u8) -> Self {
        Self {
            continuous: None,
            discrete: Some(discrete),
        }
    }

    pub fn to_action_type(&self) -> ActionType {
        if let Some(continuous) = &self.continuous {
            ActionType::Continuous(continuous.clone())
        } else if let Some(discrete) = &self.discrete {
            ActionType::Discrete(*discrete)
        } else {
            panic!("Action is not initialized")
        }
    }
}

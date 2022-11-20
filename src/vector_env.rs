use crate::action::Action;
use gym_rs::GymEnv;
pub struct VectorEnv {
    envs: Vec<Box<dyn GymEnv>>,
    pub single_action_space: usize,
    pub single_observation_space: usize,
    pub action_space: usize,
    pub observation_space: usize,
}

impl VectorEnv {
    pub fn new(
        envs: Vec<Box<dyn GymEnv>>,
        single_observation_space: usize,
        single_action_space: usize,
    ) -> Self {
        let observation_space = envs.len() * single_observation_space;
        let action_space = envs.len() * single_action_space;
        Self {
            envs,
            action_space,
            observation_space,
            single_action_space,
            single_observation_space,
        }
    }

    fn step(
        &mut self,
        actions: Vec<Action>,
    ) -> (Vec<Vec<f64>>, Vec<f64>, Vec<bool>, Vec<Option<String>>) {
        let mut states = Vec::new();
        let mut rewards = Vec::new();
        let mut dones = Vec::new();
        let mut infos = Vec::new();
        for (i, env) in self.envs.iter_mut().enumerate() {
            let (s, r, d, info) = env.step(actions[i].to_action_type());
            states.push(s);
            rewards.push(r);
            dones.push(d);
            infos.push(info);
            if d {
                let s = env.reset();
                states.push(s);
            }
        }
        (states, rewards, dones, infos)
    }

    fn reset(&mut self) -> Vec<Vec<f64>> {
        let mut states = Vec::new();
        for env in self.envs.iter_mut() {
            let s = env.reset();
            states.push(s);
        }
        states
    }

    fn render(&self, viewers: &mut Vec<&mut gym_rs::GifRender>) {
        for env in self.envs.iter() {
            env.render(viewers[0]);
        }
    }

    fn seed(&mut self, seed: u64) {
        for env in self.envs.iter_mut() {
            env.seed(seed);
        }
    }
}

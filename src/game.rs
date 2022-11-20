use gym_rs::{CartPoleEnv, GymEnv};
use crate::vector_env::VectorEnv;
fn make_env(seed: i64) -> Box<dyn GymEnv> {
    let mut env = CartPoleEnv::default();
    env.seed(u64::try_from(seed).unwrap());
    Box::new(env)
}
pub fn start_game(seed: i64, num_envs: usize) {
    let mut envs = Vec::new();
    for _ in 0..num_envs {
        envs.push(make_env(seed));
    }
    let vector_env = VectorEnv::new(envs, 4, 2);
    println!("Action space: {}", vector_env.action_space);
    println!("Observation space: {}", vector_env.observation_space);
    println!("Single action space: {}", vector_env.single_action_space);
    println!("Single observation space: {}", vector_env.single_observation_space);
}

use std::{hash::Hash, fmt::Display};

use serde::Serialize;

pub trait State: Eq + Hash + Clone + Serialize + Display{
    // Action type
    type Act: Eq + Hash + Clone + Serialize + Display;
    // The reward when agent reaches this state
    fn reward(&self) -> f64;
    // available actions to reach another state from here
    fn action_set(&self) -> Vec<Self::Act>;
    // select random action
    fn random_action(&self) -> Self::Act {
        let actions = self.action_set();
        let action = rand::random::<usize>() % actions.len();
        actions[action].clone()
    }
}



pub trait Agent<S: State> {
    // returns the current state
    fn current_state(&self) -> &S;
    // perform an action on the current state
    fn act(&mut self, action: &S::Act) -> f64;
    // randomly choose an action
    fn random_act(&mut self) -> S::Act {
        let action = self.current_state().random_action();
        action
    }
}
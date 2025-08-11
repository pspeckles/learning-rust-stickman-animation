use std::collections::HashMap;

use crate::{
    actor::{animation::AnimationFrames, squatting_animation::squatting_animation, t_pose::t_pose},
    component::animation::AnimationState,
    event::EventQueue,
};

pub struct AnimationSystem {
    human_animations: HashMap<String, AnimationFrames>,
}

impl AnimationSystem {
    pub fn new(events: &mut Box<EventQueue>) -> AnimationSystem {
        events.subscribe("animation_system");
        let mut human_animations = HashMap::new();

        human_animations.insert("squatting".to_string(), squatting_animation());

        AnimationSystem { human_animations }
    }

    pub fn apply(
        &self,
        events: &mut Box<EventQueue>,
        mut entities: Vec<&mut AnimationState>,
        dt: &u128,
    ) {
        events
            .poll("animation_system")
            .iter()
            .for_each(|s| println!("{:?}", s.name));
        let entity = entities.get_mut(0).unwrap();
        // let animation = self.human_animations.get("squatting").unwrap();

        AnimationFrames::new(vec![t_pose()]).update(entity, dt);
        // animation.update(entity, dt);
    }
}

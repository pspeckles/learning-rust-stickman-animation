use std::collections::HashMap;

use crate::{
    actor::{animation::AnimationFrames, squatting_animation::squatting_animation, t_pose::t_pose},
    component::animation::{AnimationComponent, AnimationState},
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
        human_animations.insert("t_pose".to_string(), AnimationFrames::new(vec![t_pose()]));

        AnimationSystem { human_animations }
    }

    pub fn apply(
        &self,
        events: &mut Box<EventQueue>,
        mut entities: Vec<&mut dyn AnimationComponent>,
        dt: &u128,
    ) {
        events
            .poll("animation_system")
            .iter()
            .for_each(|s| println!("{:?}", s.name));
        let entity = entities.get_mut(0).unwrap();
        let animation = self.human_animations.get("squatting").unwrap();

        animation.update(entity.get_animation_state_mut(), dt);
    }
}

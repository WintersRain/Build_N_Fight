//! Flow field pathfinding system
//!
//! Two field types:
//! - Traversal cost field: How hard is it to get there? (Cost = tile HP)
//! - Target value field: What's worth attacking? (Higher = more attractive)
//!
//! Ants pathfind using traversal costs, but choose destinations using target values.

use bevy::prelude::*;

mod breach;
mod target;
mod traversal;

pub use breach::*;
pub use target::*;
pub use traversal::*;

pub struct FlowPlugin;

impl Plugin for FlowPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TraversalField>()
            .init_resource::<TargetField>()
            .init_resource::<BreachPoints>()
            .add_event::<BreachCreatedEvent>()
            .add_systems(Update, (
                update_traversal_field,
                update_target_field,
                manage_breach_points,
            ));
    }
}

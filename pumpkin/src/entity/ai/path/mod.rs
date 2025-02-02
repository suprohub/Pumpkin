use pumpkin_protocol::client::play::CUpdateEntityPos;
use pumpkin_util::math::vector3::Vec3;

use crate::entity::living::LivingEntity;

#[derive(Default)]
pub struct Navigator {
    current_goal: Option<NavigatorGoal>,
}

pub struct NavigatorGoal {
    pub current_progress: Vec3<f64>,
    pub destination: Vec3<f64>,
    pub speed: f64,
}

impl Navigator {
    pub fn set_progress(&mut self, goal: NavigatorGoal) {
        self.current_goal = Some(goal);
    }

    pub fn cancel(&mut self) {
        self.current_goal = None;
    }

    pub async fn tick(&mut self, entity: &LivingEntity) {
        if let Some(goal) = &mut self.current_goal {
            // first lets check if we reached destination
            if goal.current_progress == goal.destination {
                // if yes, we are done here
                self.current_goal = None;
                return;
            }

            // A star algorithm
            let mut best_move = Vec3::new(0.0, 0.0, 0.0);
            let mut lowest_cost = f64::MAX;

            for x in -1..=1 {
                for z in -1..=1 {
                    let x = f64::from(x);
                    let z = f64::from(z);
                    let potential_pos = Vec3::new(
                        goal.current_progress.x + x,
                        goal.current_progress.y,
                        goal.current_progress.z + z,
                    );

                    let node = Node::new(potential_pos);
                    let cost = node.get_expense(goal.destination);

                    if cost < lowest_cost {
                        lowest_cost = cost;
                        best_move = Vec3::new(x, 0.0, z);
                    }
                }
            }

            // this is important, first this saves us many packets when we don't actually move, and second this prevents division using zero
            // when normalize
            if best_move.x == 0.0 && best_move.z == 0.0 {
                return;
            }
            // Update current progress based on the best move
            goal.current_progress += best_move.normalize() * goal.speed;

            // now lets move
            entity.set_pos(goal.current_progress);
            let pos = entity.entity.pos.load();
            let last_pos = entity.last_pos.load();

            entity
                .entity
                .world
                .broadcast_packet_all(&CUpdateEntityPos::new(
                    entity.entity.entity_id.into(),
                    Vec3::new(
                        pos.x.mul_add(4096.0, -(last_pos.x * 4096.0)) as i16,
                        pos.y.mul_add(4096.0, -(last_pos.y * 4096.0)) as i16,
                        pos.z.mul_add(4096.0, -(last_pos.z * 4096.0)) as i16,
                    ),
                    entity
                        .entity
                        .on_ground
                        .load(std::sync::atomic::Ordering::Relaxed),
                ))
                .await;
        }
    }
}

pub struct Node {
    pub location: Vec3<f64>,
}

impl Node {
    #[must_use]
    pub fn new(location: Vec3<f64>) -> Self {
        Self { location }
    }
    /// How expensive is it to go to a location
    ///
    /// Returns a f64, Higher = More Expensive
    #[must_use]
    pub fn get_expense(&self, end: Vec3<f64>) -> f64 {
        self.location.squared_distance_to_vec(end).sqrt()
    }
}

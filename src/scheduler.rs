use spawning_pool::EntityId;
use components;

#[derive(Serialize, Deserialize)]
pub struct Scheduler {
    pub entities: Vec<EntityId>,
    pub current_index: usize
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            entities: vec![],
            current_index: 0 as usize
        }
    }

    pub fn add_entity(&mut self, entity: EntityId, spawning_pool: &components::SpawningPool) {
        if spawning_pool.get::<components::Controller>(entity).is_some() {
            self.entities.push(entity);
        }
    }

    pub fn get_current(&self) -> EntityId {
        self.entities[self.current_index]
    }

    pub fn remove_entity(&mut self) {
        self.entities.remove(self.current_index);
        self.current_index %= self.entities.len();
    }

    pub fn tick(&mut self) {
        self.current_index = (self.current_index + 1) % self.entities.len();
    }
}

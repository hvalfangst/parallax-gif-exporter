use crate::state::structs::State;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn execute_core_logic(game_state: &mut State, core_logic_operations: &HashMap<String, Rc<RefCell<dyn CoreLogic>>>) {
    for (_, core_logic_operation) in core_logic_operations.iter() {
        core_logic_operation.borrow().execute(game_state);
    }
}

pub trait CoreLogic {
    fn execute(&self, game_state: &mut State);
}

pub struct AlternateHeartSpriteFrames;


pub struct AlternateGroundSpriteFrames;

impl CoreLogic for AlternateGroundSpriteFrames {
    fn execute(&self, game_state: &mut State) {
        // Alternate between the grass sprite frames every 200 milliseconds
        if game_state.last_ground_sprite_frame_index_change.elapsed() >= std::time::Duration::from_millis(200) {
            game_state.ground_sprite_frame_index = (game_state.ground_sprite_frame_index + 1) % 2; // Cycle between 0 and 1
            game_state.last_ground_sprite_frame_index_change = std::time::Instant::now(); // Reset the timer to current time
        }
    }
}

pub struct AlternateLightHouseSpriteFrames;

impl CoreLogic for AlternateLightHouseSpriteFrames {
    fn execute(&self, game_state: &mut State) {
        // Alternate between the lighthouse sprite frames every 200 milliseconds
        if game_state.last_light_house_sprite_frame_index_change.elapsed() >= std::time::Duration::from_millis(750) {
            game_state.lighthouse_sprite_frame_index = (game_state.lighthouse_sprite_frame_index + 1) % 4; // Cycle between 0 and 3
            game_state.last_light_house_sprite_frame_index_change = std::time::Instant::now(); // Reset the timer to current time
        }
    }
}


pub fn initialize_core_logic_map() -> HashMap<String, Rc<RefCell<dyn CoreLogic>>> {
    let mut logic_map: HashMap<String, Rc<RefCell<dyn CoreLogic>>> = HashMap::new();

    logic_map.insert("AlternateGroundSpriteFrames".to_string(), Rc::new(RefCell::new(AlternateGroundSpriteFrames)));
    logic_map.insert("AlternateLightHouseSprites".to_string(), Rc::new(RefCell::new(AlternateLightHouseSpriteFrames)));

    logic_map
}

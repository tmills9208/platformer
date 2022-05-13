/* World.rs

Requirements for EACH world/level:
- load parallax bg / effects
- load level (ldtk / 2d physics)
- load player character
- load other characters
- load npc's
- load other entities (entry points, interactables, etc.)

- run and update world
- detect boss/bosses and update ui for boss healthbars
*/

use bevy::prelude::*;

pub struct World;

impl FromWorld for World {

}
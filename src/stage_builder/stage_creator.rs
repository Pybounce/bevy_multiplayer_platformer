use super::stage_asset::Stage;
use bevy::prelude::*;


pub struct StageCreator<'a> {
    stage: &'a Stage, 
    colour_palettes: &'a Handle<Image>
}


impl<'a> StageCreator<'a> {

    pub fn new(stage: &'a Stage, colour_palettes: &'a Handle<Image>) -> Self {
        StageCreator {
            stage,
            colour_palettes
        }
    }

    pub fn build(&self) -> bool {
        build_perimeter(self) && build_background(self)
    }


}

fn build_perimeter(stage_creator: &StageCreator) -> bool {
    for i in 0..stage_creator.stage.tiles_width {
        
    }

    return true;
}

fn build_background(stage_creator: &StageCreator) -> bool {
    todo!();
}

fn set_camera_background(texture_handle: &Handle<Image>) {
    todo!();
}
use bevy::prelude::*;
use bevy_dexterous_developer::*;

reloadable_main!((initial_plugins){
    App::new()
        .add_plugins((initial_plugins.initialize::<DefaultPlugins>(), bevy_hot_min::plugin))
        .run();
});

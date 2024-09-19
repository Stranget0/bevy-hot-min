use bevy::{color::palettes::tailwind, prelude::*};
use bevy_dexterous_developer::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera_2d)
        .setup_reloadable_elements::<reloadable>();
}

reloadable_scope!(reloadable(app) {
    app.reset_setup::<Person,_>((
        add_people,
        log_people.after(add_people),
        display_people.after(add_people),
    )).clear_marked_on_reload::<Person>();
});

#[derive(Component)]
struct Person;

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name::new("Alice")));
    commands.spawn((Person, Name::new("Bob")));
    commands.spawn((Person, Name::new("Charlie")));
}

fn log_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("Added person: {:?}", name);
    }
}

fn display_people(mut commands: Commands, query: Query<&Name, With<Person>>) {
    let node = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                row_gap: Val::Px(8.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: tailwind::GRAY_950.into(),
            ..default()
        })
        .id();

    for name in query.iter() {
        commands
            .spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    ..Default::default()
                },
                text: Text::from_section(
                    name.to_string(),
                    TextStyle {
                        font_size: 40.0,
                        color: tailwind::GRAY_50.into(),
                        ..default()
                    },
                ),
                ..Default::default()
            })
            .set_parent(node);
    }
}

fn spawn_camera_2d(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}

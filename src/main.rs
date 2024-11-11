use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    time::common_conditions::once_after_delay,
    utils::Duration,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::DEBUG,
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spawn_text.run_if(once_after_delay(Duration::from_secs(1))),
                spawn_text.run_if(once_after_delay(Duration::from_secs(2))),
            ),
        )
        .run();
}

#[derive(Component)]
struct ParentNode;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.),
                ..Default::default()
            },
            ParentNode,
        ))
        .with_children(|parent| {
            parent.spawn(Text::new(
                "2 children will be generated at 1 and 2 seconds after the app starts",
            ));
        });
}

fn spawn_text(
    mut commands: Commands,
    parent_query: Single<Entity, With<ParentNode>>,
    asset_server: Res<AssetServer>,
) {
    commands.entity(*parent_query).with_children(|parent| {
        parent
            .spawn(Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn((
                    Text::new("NotoSansJA => こんにちは世界"),
                    TextFont {
                        font: asset_server.load("NotoSansJA.ttf"),
                        ..default()
                    },
                ));
                parent.spawn(Text::new("Bevy Font => こんにちは世界"));
            });
    });
}

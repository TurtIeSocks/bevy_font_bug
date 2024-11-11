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
        .init_resource::<FontAsset>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spawn_text.run_if(on_event::<AssetEvent<Font>>),
                spawn_text.run_if(once_after_delay(Duration::from_secs(2))),
            ),
        )
        .run();
}

#[derive(Component)]
struct ParentNode;

#[derive(Default, Resource)]
struct FontAsset(Handle<Font>);

fn setup(
    asset_server: Res<AssetServer>,
    mut font_asset: ResMut<FontAsset>,
    mut commands: Commands,
) {
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
        .with_children(|parent: &mut ChildBuilder<'_>| {
            parent.spawn(Text::new(
                "Each child is generated on a font asset event and the last one loads after a 2s delay",
            ));
        });
    font_asset.0 = asset_server.load("NotoSansJA.ttf");
}

fn spawn_text(
    mut commands: Commands,
    font_asset: Res<FontAsset>,
    parent_query: Single<Entity, With<ParentNode>>,
    mut er: EventReader<AssetEvent<Font>>,
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
                let is_event_triggered = er.len() > 0;
                for event in er.read() {
                    let is_added = event.is_added(font_asset.0.id());
                    let is_loaded = event.is_loaded_with_dependencies(font_asset.0.id());

                    // if !is_added && !is_loaded {
                    // // if you uncomment this, all text assets will use the NotoSansJA font?
                    //     return;
                    // }

                    parent.spawn(Text::new(format!(
                        "NotoSansJA asset, is_added: {}, is_loaded: {}",
                        is_added, is_loaded
                    )));
                }
                if !is_event_triggered {
                    parent.spawn(Text::new("2 sec delay"));
                }

                parent.spawn((
                    Text::new("NotoSansJA => こんにちは世界"),
                    TextFont {
                        font: font_asset.0.clone(),
                        ..default()
                    },
                ));
                parent.spawn(Text::new("Bevy Font => こんにちは世界"));
            });
    });
}

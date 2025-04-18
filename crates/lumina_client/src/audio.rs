use bevy::audio::*;
use bevy::prelude::*;
use lumina_shared::prelude::*;

use crate::screens::Screen;

pub(super) struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        // Prespawn the bg music entity
        app.world_mut().spawn(BgMusic);

        app.add_systems(OnEnter(Screen::MainMenu), play_main_menu_bg)
            .add_systems(OnEnter(Screen::InGame), play_in_game_bg)
            .add_systems(Update, button_interaction)
            .observe(fire_ammo)
            .observe(ammo_hit);
    }
}

fn button_interaction(
    mut commands: Commands,
    q_interactions: Query<&Interaction, Changed<Interaction>>,
    asset_server: Res<AssetServer>,
) {
    for interaction in q_interactions.iter() {
        let audio_name = match interaction {
            Interaction::Pressed => "audio/ButtonClick.ogg",
            Interaction::Hovered => "audio/ButtonHover.ogg",
            Interaction::None => continue,
        };

        commands.spawn(AudioBundle {
            source: asset_server.load(audio_name),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

fn fire_ammo(trigger: Trigger<FireAmmo>, mut commands: Commands, asset_server: Res<AssetServer>) {
    let fire_ammo = trigger.event();
    let position = fire_ammo.position;
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/Cannon.ogg"),
            settings: PlaybackSettings::DESPAWN.with_spatial(true),
        },
        TransformBundle::from_transform(Transform::from_xyz(position.x, position.y, 0.0)),
    ));
}

fn ammo_hit(trigger: Trigger<AmmoHit>, mut commands: Commands, asset_server: Res<AssetServer>) {
    let position = trigger.event();
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/AmmoHit.ogg"),
            settings: PlaybackSettings::DESPAWN.with_spatial(true),
        },
        TransformBundle::from_transform(Transform::from_xyz(position.x, position.y, 0.0)),
    ));
}

fn play_main_menu_bg(
    mut commands: Commands,
    q_bg_music: Query<Entity, With<BgMusic>>,
    asset_server: Res<AssetServer>,
) {
    let entity = q_bg_music.single();
    commands.entity(entity).despawn();

    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/bg_music/MainMenu.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new(0.5),
                ..default()
            },
        },
        BgMusic,
    ));
}

fn play_in_game_bg(
    mut commands: Commands,
    q_bg_music: Query<Entity, With<BgMusic>>,
    asset_server: Res<AssetServer>,
) {
    let entity = q_bg_music.single();
    commands.entity(entity).despawn();

    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/bg_music/InGame.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new(0.5),
                ..default()
            },
        },
        BgMusic,
    ));
}

#[derive(Component)]
pub struct BgMusic;

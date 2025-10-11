use std::ops::RangeInclusive;

use bevy::prelude::*;
use bevy::window::WindowResized;

use crate::AppState;
use crate::plugins::Board;
use crate::utils::ProjectionExt;

#[derive(Component)]
pub struct MainCamera;

fn spawn_main(mut commands: Commands) { commands.spawn((Camera2d, MainCamera)); }

#[derive(Resource)]
pub struct CameraLimits {
    horizontal: RangeInclusive<f32>,
    vertical: RangeInclusive<f32>,
    scale: RangeInclusive<f32>,
}

impl Default for CameraLimits {
    fn default() -> Self { Self { horizontal: 0.0..=0.0, vertical: 0.0..=0.0, scale: 0.0..=0.0 } }
}

impl CameraLimits {
    fn calculate_scale_limits(&mut self, board_size: Vec2, window_size: Vec2) {
        let min_board = Vec2::splat(3.0);
        let max_board = board_size + 0.5;
        let min_scales = min_board / window_size;
        let max_scales = max_board / window_size;
        self.scale = min_scales.max_element()..=max_scales.max_element();
    }

    fn calculate_translation_limits(&mut self, board_size: Vec2, window_size: Vec2, zoom: f32) {
        let board_min = Vec2::new(-0.25, -board_size.y - 0.25);
        let board_max = Vec2::new(board_size.x + 0.25, 0.25);
        let center = Vec2::new(board_size.x / 2.0, -board_size.y / 2.0);
        let half_view = window_size / 2.0 * zoom;
        let cam_max = board_max - half_view;
        let cam_min = board_min + half_view;
        self.horizontal =
            if cam_min.x <= cam_max.x { cam_min.x..=cam_max.x } else { center.x..=center.x };
        self.vertical =
            if cam_min.y <= cam_max.y { cam_min.y..=cam_max.y } else { center.y..=center.y };
    }

    pub fn limit_scale(&self, scale: &mut f32) {
        *scale = scale.max(*self.scale.start());
        *scale = scale.min(*self.scale.end());
    }

    pub fn limit_translation(&self, translation: &mut Vec3) {
        translation.x = translation.x.max(*self.horizontal.start());
        translation.x = translation.x.min(*self.horizontal.end());
        translation.y = translation.y.max(*self.vertical.start());
        translation.y = translation.y.min(*self.vertical.end());
    }
}

fn spawn(
    camera: Single<(&mut Transform, &mut Projection), With<MainCamera>>, window: Single<&Window>,
    board: Res<Board>, mut commands: Commands,
) {
    let (mut transform, mut projection) = camera.into_inner();
    let orthographic = projection.as_orthographic_mut().unwrap();
    let board_size = board.size();
    let window_size = window.size();
    let mut limits = CameraLimits::default();
    limits.calculate_scale_limits(board_size, window_size);
    orthographic.scale = *limits.scale.end();
    limits.calculate_translation_limits(board_size, window_size, orthographic.scale);
    limits.limit_translation(&mut transform.translation);
    commands.insert_resource(limits);
}

fn window_resized(
    mut resize_events: MessageReader<WindowResized>,
    camera: Single<&mut Projection, With<MainCamera>>, mut limit: ResMut<CameraLimits>,
    board: Res<Board>,
) {
    let Some(window) = resize_events.read().last() else { return };
    let mut projection = camera.into_inner();
    let orthographic = projection.as_orthographic_mut().unwrap();
    let window_size = Vec2::new(window.width, window.height);
    limit.calculate_scale_limits(board.size(), window_size);
    limit.limit_scale(&mut orthographic.scale);
}

fn scale_changed(
    camera: Single<(&mut Transform, &Projection), (Changed<Projection>, With<MainCamera>)>,
    window: Single<&Window>, mut limit: ResMut<CameraLimits>, board: Res<Board>,
) {
    let (mut transform, projection) = camera.into_inner();
    let orthographic = projection.as_orthographic().unwrap();
    limit.calculate_translation_limits(board.size(), window.size(), orthographic.scale);
    limit.limit_translation(&mut transform.translation);
}

fn despawn(mut commands: Commands) { commands.remove_resource::<CameraLimits>(); }

pub fn camera(app: &mut App) {
    app.add_systems(Startup, spawn_main)
        .add_systems(OnEnter(AppState::Playing), spawn)
        .add_systems(OnExit(AppState::Playing), spawn)
        .add_systems(Update, (window_resized, scale_changed).run_if(in_state(AppState::Playing)))
        .add_systems(OnEnter(AppState::Menu), despawn);
}

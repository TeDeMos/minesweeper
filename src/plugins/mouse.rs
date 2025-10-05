use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::prelude::*;

use crate::AppState;
use crate::plugins::{Board, CameraLimits, MainCamera};
use crate::utils::ProjectionExt;

#[derive(Resource, Default)]
struct MouseState {
    dragging: bool,
    cursor_start: Vec2,
    world_start: Vec2,
    translation_start: Vec3,
    global_start: GlobalTransform,
}

impl MouseState {
    fn set(
        &mut self, cursor: Vec2, transform: Transform, global: &GlobalTransform, camera: &Camera,
    ) {
        self.dragging = false;
        self.cursor_start = cursor;
        self.world_start = camera.viewport_to_world_2d(global, cursor).unwrap();
        self.translation_start = transform.translation;
        self.global_start = *global;
    }

    fn check_dragging(&mut self, cursor: Vec2) -> bool {
        self.dragging = self.dragging || (cursor - self.cursor_start).length_squared() >= 25.0;
        self.dragging
    }

    fn get_translation(&self, cursor: Vec2, camera: &Camera) -> Vec3 {
        let end = camera.viewport_to_world_2d(&self.global_start, cursor).unwrap();
        self.translation_start - (end - self.world_start).extend(0.0)
    }
}

#[derive(EntityEvent)]
pub struct LeftClicked {
    entity: Entity,
}

#[derive(EntityEvent)]
pub struct RightClicked {
    entity: Entity,
}

fn spawn(mut commands: Commands) { commands.insert_resource(MouseState::default()); }

fn click_event(
    mut commands: Commands,
    camera: Single<(&mut Transform, &GlobalTransform, &Camera), With<Camera2d>>,
    window: Single<&mut Window>, mut state: ResMut<MouseState>,
    button: Res<ButtonInput<MouseButton>>, board: Res<Board>, limits: Res<CameraLimits>,
) {
    let Some(cursor) = window.cursor_position() else { return };
    let (mut transform, global, camera) = camera.into_inner();
    let cursor_world = camera.viewport_to_world_2d(global, cursor).unwrap();
    let entity = board.get_from_world(cursor_world);
    if button.just_pressed(MouseButton::Left) && !button.pressed(MouseButton::Right) {
        state.set(cursor, *transform, global, camera);
    } else if button.pressed(MouseButton::Left) && state.check_dragging(cursor) {
        transform.translation = state.get_translation(cursor, camera);
        limits.limit_translation(&mut transform.translation);
    } else if button.just_released(MouseButton::Left)
        && !state.dragging
        && let Some(entity) = entity
    {
        commands.trigger(LeftClicked { entity });
    } else if button.just_pressed(MouseButton::Right)
        && let Some(entity) = entity
    {
        commands.trigger(RightClicked { entity });
    }
}

fn scroll_event(
    camera: Single<(&mut Transform, &mut Projection, &GlobalTransform, &Camera), With<MainCamera>>,
    window: Single<&Window>, scroll: Res<AccumulatedMouseScroll>, limits: Res<CameraLimits>,
) {
    if scroll.delta.y == 0.0 {
        return;
    }
    let Some(cursor) = window.cursor_position() else { return };
    let (mut transform, mut projection, global, camera) = camera.into_inner();
    let mouse_world = camera.viewport_to_world_2d(global, cursor).unwrap().extend(0.0);
    let orthographic = projection.as_orthographic_mut().unwrap();
    let change = 1.2f32.powf(-scroll.delta.y);
    let old = orthographic.scale;
    orthographic.scale *= change;
    limits.limit_scale(&mut orthographic.scale);
    transform.translation =
        mouse_world - (mouse_world - transform.translation) * orthographic.scale / old;
}

fn despawn(mut commands: Commands) { commands.remove_resource::<MouseState>(); }

pub fn mouse(app: &mut App) {
    app.add_systems(OnEnter(AppState::Playing), spawn)
        .add_systems(Update, (click_event, scroll_event).run_if(in_state(AppState::Playing)))
        .add_systems(OnEnter(AppState::Menu), despawn);
}

use bevy::prelude::*;
use sbepistats::*;

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatsPlugin))
        .add_stat_type::<Speed>()
        .add_stat_modifier_add::<Speed, SpeedBoost>()
        .add_stat_modifier_add::<Speed, SpeedBooster>()
        .add_stat_type::<PowerLevel>()
        .add_stat_modifier_add::<PowerLevel, PowerUp>();
    app
}

#[derive(StatType)]
struct Speed;

#[derive(Component)]
struct SpeedBoost;

impl StatModifierAdd<Speed> for SpeedBoost {
    fn add(&self) -> f32 {
        0.2
    }
}

#[derive(Component)]
struct SpeedBooster;

impl StatModifierAdd<Speed> for SpeedBooster {
    fn add(&self) -> f32 {
        0.3
    }
}

#[derive(StatType)]
#[stat_type(u32)]
struct PowerLevel;

#[derive(Component)]
struct PowerUp;

impl StatModifierAdd<PowerLevel> for PowerUp {
    fn add(&self) -> u32 {
        1
    }
}

#[test]
fn f32_stat_without_modifier() {
    let mut app = app();
    app.world_mut().spawn(Stat::<Speed>::new(1.0));
    app.update();
    assert_eq!(
        1.0,
        app.world_mut()
            .query::<&Stat::<Speed>>()
            .single(app.world())
            .unwrap()
            .total()
    )
}

#[test]
fn f32_stat_with_modifier() {
    let mut app = app();
    app.world_mut().spawn((Stat::<Speed>::new(1.0), SpeedBoost));
    app.update();
    assert_eq!(
        1.2,
        app.world_mut()
            .query::<&Stat::<Speed>>()
            .single(app.world())
            .unwrap()
            .total()
    )
}

#[test]
fn f32_stat_with_two_modifier() {
    let mut app = app();
    app.world_mut()
        .spawn((Stat::<Speed>::new(1.0), SpeedBoost, SpeedBooster));
    app.update();
    assert_eq!(
        1.5,
        app.world_mut()
            .query::<&Stat::<Speed>>()
            .single(app.world())
            .unwrap()
            .total()
    )
}

#[test]
fn u32_stat_without_modifier() {
    let mut app = app();
    app.world_mut().spawn(Stat::<PowerLevel>::new(1));
    app.update();
    assert_eq!(
        1,
        app.world_mut()
            .query::<&Stat::<PowerLevel>>()
            .single(app.world())
            .unwrap()
            .total()
    )
}

#[test]
fn u32_stat_with_modifier() {
    let mut app = app();
    app.world_mut().spawn((Stat::<PowerLevel>::new(1), PowerUp));
    app.update();
    assert_eq!(
        2,
        app.world_mut()
            .query::<&Stat::<PowerLevel>>()
            .single(app.world())
            .unwrap()
            .total()
    )
}

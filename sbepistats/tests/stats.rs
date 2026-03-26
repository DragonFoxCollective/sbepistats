use bevy::prelude::*;
use sbepistats::*;

fn app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_stat_type::<Speed>()
        .add_stat_modifier::<Speed, SpeedBoost>()
        .add_stat_type::<PowerLevel>();
    app
}

#[derive(StatType)]
struct Speed;

#[derive(Component)]
struct SpeedBoost;

impl StatModifier<Speed> for SpeedBoost {
    fn multiply_after(&self) -> f32 {
        0.2
    }
}

#[derive(StatType)]
#[stat_type(u32)]
struct PowerLevel;

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

use std::time::Duration;

use bevy::prelude::*;
use sbepistats::*;

fn app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_stat_type::<Speed>()
        .configure_stat_type_add::<Speed>()
        .add_systems(
            PreUpdate,
            speed_charge.in_set(StatSystems::<Speed>::Op(DataTypeOp::Add)),
        )
        .add_stat_type::<SpeedChargeMultiplier>()
        .add_stat_modifier_add::<SpeedChargeMultiplier, SpeedChargeMultBoost>()
        .order_stats::<SpeedChargeMultiplier, Speed>();
    app
}

#[derive(StatType)]
struct SpeedChargeMultiplier;

#[derive(Component)]
struct SpeedChargeMultBoost;

impl StatModifierAdd<SpeedChargeMultiplier> for SpeedChargeMultBoost {
    fn add(&self) -> f32 {
        1.0
    }
}

#[derive(StatType)]
struct Speed;

#[derive(Component)]
struct SpeedCharge(Duration);

fn speed_charge(mut query: Query<(&mut Stat<Speed>, &SpeedCharge, &Stat<SpeedChargeMultiplier>)>) {
    for (mut stat, charge, mult) in query.iter_mut() {
        stat.add_modifier(charge.0.as_secs_f32() * mult.total());
    }
}

#[test]
fn derived_stat() {
    let mut app = app();
    app.world_mut().spawn((
        Stat::<Speed>::new(1.0),
        Stat::<SpeedChargeMultiplier>::new(1.0),
        SpeedCharge(Duration::from_secs_f32(2.0)),
    ));
    app.update();
    assert_eq!(
        3.0,
        app.world_mut()
            .query::<&Stat::<Speed>>()
            .single(app.world())
            .unwrap()
            .total()
    )
}

#[test]
fn derived_stat_with_modifier() {
    let mut app = app();
    app.world_mut().spawn((
        Stat::<Speed>::new(1.0),
        Stat::<SpeedChargeMultiplier>::new(1.0),
        SpeedCharge(Duration::from_secs_f32(2.0)),
        SpeedChargeMultBoost,
    ));
    app.update();
    assert_eq!(
        5.0,
        app.world_mut()
            .query::<&Stat::<Speed>>()
            .single(app.world())
            .unwrap()
            .total()
    )
}

use bevy::prelude::*;
pub use sbepistats_derive::*;

pub trait StatType {
    type DataType;
}

pub trait Add {
    fn add(self, rhs: Self) -> Self;
}

impl<T: std::ops::Add<T, Output = T>> Add for T {
    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}

pub trait Mul {
    fn mul(self, rhs: Self) -> Self;
}

impl<T: std::ops::Mul<T, Output = T>> Mul for T {
    fn mul(self, rhs: Self) -> Self {
        self * rhs
    }
}

pub trait Zero {
    fn zero() -> Self;
}

impl<T: num_traits::Zero> Zero for T {
    fn zero() -> Self {
        num_traits::Zero::zero()
    }
}

pub trait One {
    fn one() -> Self;
}

impl<T: num_traits::One> One for T {
    fn one() -> Self {
        num_traits::One::one()
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum DataTypeOp {
    Add,
    MulBefore,
    MulAfter,
}

#[derive(Component)]
pub struct Stat<T: StatType> {
    base: T::DataType,
    running_total: T::DataType,
    running_op_total: T::DataType,
}

impl<T: StatType<DataType: Clone + Zero>> Stat<T> {
    pub fn new(base: T::DataType) -> Self {
        Stat {
            base: base.clone(),
            running_total: base,
            running_op_total: Zero::zero(),
        }
    }
}

impl<T: StatType<DataType: Clone>> Stat<T> {
    fn clear(&mut self) {
        self.running_total = self.base.clone();
    }

    pub fn total(&self) -> T::DataType {
        self.running_total.clone()
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum StatsSystems {
    Clear,
    Op(DataTypeOp),
    Apply(DataTypeOp),
}

fn clear_stat<T: StatType<DataType: Clone + Send + Sync + 'static> + Send + Sync + 'static>(
    mut stats: Query<&mut Stat<T>>,
) {
    println!("clearing {}", ShortName::of::<T>());
    for mut stat in stats.iter_mut() {
        stat.clear();
    }
}

fn apply_add<
    T: StatType<DataType: std::fmt::Debug + Zero + Add + Clone + Send + Sync + 'static>
        + Send
        + Sync
        + 'static,
>(
    mut stats: Query<&mut Stat<T>>,
) {
    println!("starting apply {}", ShortName::of::<T>());
    for mut stat in stats.iter_mut() {
        stat.running_total = stat
            .running_total
            .clone()
            .add(stat.running_op_total.clone());
        stat.running_op_total = Zero::zero();
    }
}

pub trait AppExt {
    fn add_stat_type<T: StatType<DataType: Clone + Send + Sync + 'static> + Send + Sync + 'static>(
        &mut self,
    ) -> &mut Self;

    fn add_stat_modifier_add<
        T: StatType<DataType: std::fmt::Debug + Zero + Add + Clone + Send + Sync + 'static>
            + Send
            + Sync
            + 'static,
        Modifier: Component + 'static,
    >(
        &mut self,
        func: impl Fn(&Modifier) -> T::DataType + Send + Sync + 'static,
    ) -> &mut Self;
}

impl AppExt for App {
    fn add_stat_type<
        T: StatType<DataType: Clone + Send + Sync + 'static> + Send + Sync + 'static,
    >(
        &mut self,
    ) -> &mut Self {
        self.add_systems(PreUpdate, clear_stat::<T>.in_set(StatsSystems::Clear));
        self
    }

    fn add_stat_modifier_add<
        T: StatType<DataType: std::fmt::Debug + Zero + Add + Clone + Send + Sync + 'static>
            + Send
            + Sync
            + 'static,
        Modifier: Component + 'static,
    >(
        &mut self,
        func: impl Fn(&Modifier) -> T::DataType + Send + Sync + 'static,
    ) -> &mut Self {
        self.add_systems(
            PreUpdate,
            (
                (move |mut stats: Query<(&mut Stat<T>, &Modifier)>| {
                    for (mut stat, modifier) in stats.iter_mut() {
                        stat.running_op_total = stat.running_op_total.clone().add(func(modifier));
                    }
                })
                .in_set(StatsSystems::Op(DataTypeOp::Add)),
                apply_add::<T>.in_set(StatsSystems::Apply(DataTypeOp::Add)),
            ),
        );
        self
    }
}

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PreUpdate,
            (
                StatsSystems::Clear,
                StatsSystems::Op(DataTypeOp::MulBefore),
                StatsSystems::Apply(DataTypeOp::MulBefore),
                StatsSystems::Op(DataTypeOp::Add),
                StatsSystems::Apply(DataTypeOp::Add),
                StatsSystems::Op(DataTypeOp::MulAfter),
                StatsSystems::Apply(DataTypeOp::MulAfter),
            )
                .chain(),
        );
    }
}

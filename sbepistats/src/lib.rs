//! A Minecraft-inspired stat system for Bevy.
//!
//! To get started, add the [`StatsPlugin`] plugin to your app.
//! Then, derive [`StatType`] on a struct and register it to your app with [`AppExt::add_stat_type`].
//! Finally, impl [`StatModifierAdd`] or [`StatModifierMul`] on a component and
//! register it to your app with [`AppExt::add_stat_modifier_add`] or [`AppExt::add_stat_modifier_mul`].
//!
//! To use the stats, add a [`Stat`] component to an entity, and any stat modifiers added to it will be reflected in [`Stat::total`].
//!
//! Stat datatypes are flexible, hence the separation of [`Add`] and [`Mul`]. If you need multiplication for a datatype that either
//! can't multiply or does it in an unwanted way, consider using a wrapper type. At minimum, a stat datatype requires [`Add`].
//!
//! If you're using [`bevy_auto_plugin`](::bevy_auto_plugin), the build hooks [`StatTypeHook`], [`StatModifierAddHook`], and [`StatModifierMulHook`] are available.
//!
//! # Example
//!
//! ```rust
//! # use bevy::prelude::*;
//! # use sbepistats::*;
//! #
//! fn main() {
//!     App::new()
//!         .add_plugins((DefaultPlugins, StatsPlugin))
//!         .add_stat_type::<Speed>()
//!         .add_stat_modifier_add::<Speed, SpeedBoost>()
//!         .add_systems(Startup, |mut commands: Commands| {
//!             commands.spawn((Stat::<Speed>::new(1.0), SpeedBoost));
//!         });
//! }
//!
//! #[derive(StatType)]
//! struct Speed;
//!
//! #[derive(Component)]
//! struct SpeedBoost;
//!
//! impl StatModifierAdd<Speed> for SpeedBoost {
//!     fn add(&self) -> f32 {
//!         0.2
//!     }
//! }
//! ```

use bevy::prelude::*;
#[cfg(feature = "bevy_auto_plugin")]
pub use bevy_auto_plugin::*;

/// Derive macro for [`StatType`].
///
/// Has a `stat_type` attribute that takes a type for the [`StatType::DataType`], which defaults to [`f32`].
///
/// ```rust
/// # use sbepistats::*;
/// #[derive(StatType)]
/// #[stat_type(u32)]
/// struct MyStat;
/// ```
pub use sbepistats_derive::StatType;

#[cfg(feature = "bevy_auto_plugin")]
mod bevy_auto_plugin;

/// Marker trait for defining a unique stat.
///
/// Must be registered with [`AppExt::add_stat_type`].
///
/// ```rust
/// # use sbepistats::*;
/// #[derive(StatType)]
/// #[stat_type(u32)]
/// struct MyStat;
/// ```
pub trait StatType {
    /// What type the stat uses.
    ///
    /// The derive defaults to [`f32`].
    type DataType;
}

/// Stat datatypes that can do addition, eg [`f32`] and [`std::time::Duration`].
pub trait Add {
    /// Addition.
    fn add(self, rhs: Self) -> Self;

    /// Additive identity.
    fn zero() -> Self;
}

impl<T: std::ops::Add<T, Output = T> + num_traits::Zero> Add for T {
    fn add(self, rhs: Self) -> Self {
        self + rhs
    }

    fn zero() -> Self {
        num_traits::Zero::zero()
    }
}

/// Stat datatypes that can do multiplication, eg [`f32`].
pub trait Mul {
    /// Multiplication.
    fn mul(self, rhs: Self) -> Self;

    /// Multiplicative identity.
    fn one() -> Self;
}

impl<T: std::ops::Mul<T, Output = T> + num_traits::One> Mul for T {
    fn mul(self, rhs: Self) -> Self {
        self * rhs
    }

    fn one() -> Self {
        num_traits::One::one()
    }
}

/// Representations of possible stat modifier operations.
///
/// Used in [`StatsSystems`].
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum DataTypeOp {
    Add,
    MulBefore,
    MulAfter,
}

/// Component containing the [`StatType`]'s values.
///
/// ```rust
/// # use bevy::prelude::*;
/// # use sbepistats::*;
/// #
/// # #[derive(StatType)]
/// # struct MyStat;
/// #
/// fn my_system(stats: Query<&Stat<MyStat>>) {
///     for stat in stats.iter() {
///         println!("Stat total: {}", stat.total());
///     }
/// }
/// ```
#[derive(Component)]
pub struct Stat<T: StatType> {
    base: T::DataType,
    running_total: T::DataType,
    running_op_total: T::DataType,
}

impl<T: StatType<DataType: Clone + Add>> Stat<T> {
    pub fn new(base: T::DataType) -> Self {
        Stat {
            base: base.clone(),
            running_total: base,
            running_op_total: Add::zero(),
        }
    }
}

impl<T: StatType<DataType: Clone>> Stat<T> {
    fn clear(&mut self) {
        self.running_total = self.base.clone();
    }

    /// The base stat value.
    pub fn base(&self) -> T::DataType {
        self.base.clone()
    }

    /// The stat value after all modifiers.
    ///
    /// Updates in [`PreUpdate`].
    pub fn total(&self) -> T::DataType {
        self.running_total.clone()
    }
}

/// A modifier to a [`Stat`] that adds to it.
///
/// Must be registered with [`AppExt::add_stat_modifier_add`].
///
/// ```rust
/// # use bevy::prelude::*;
/// # use sbepistats::*;
/// #
/// # #[derive(StatType)]
/// # struct MyStat;
/// #
/// #[derive(Component)]
/// struct MyStatModifier;
///
/// impl StatModifierAdd<MyStat> for MyStatModifier {
///     fn add(&self) -> f32 {
///         0.2
///     }
/// }
/// ```
pub trait StatModifierAdd<T: StatType<DataType: Add>> {
    /// Addition to the total, after [`StatModifierMul::mul_before`] but before [`StatModifierMul::mul_after`].
    fn add(&self) -> T::DataType {
        Add::zero()
    }
}

/// A modifier to a [`Stat`] that multiplies to it.
///
/// Must be registered with [`AppExt::add_stat_modifier_mul`].
///
/// ```rust
/// # use bevy::prelude::*;
/// # use sbepistats::*;
/// #
/// # #[derive(StatType)]
/// # struct MyStat;
/// #
/// #[derive(Component)]
/// struct MyStatModifier;
///
/// impl StatModifierMul<MyStat> for MyStatModifier {
///     fn mul_before(&self) -> f32 {
///         0.2
///     }
///     fn mul_after(&self) -> f32 {
///         0.2
///     }
/// }
/// ```
pub trait StatModifierMul<T: StatType<DataType: Add + Mul>> {
    /// Multiplication to the base, before [`StatModifierAdd::add`].
    fn mul_before(&self) -> T::DataType {
        Add::zero()
    }

    /// Multiplication to the total after [`StatModifierAdd::add`] and [`StatModifierMul::mul_before`].
    fn mul_after(&self) -> T::DataType {
        Add::zero()
    }
}

/// System ordering for stat systems.
///
/// Set up in [`StatsPlugin`].
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum StatsSystems {
    Clear,
    Op(DataTypeOp),
    Apply(DataTypeOp),
}

fn clear_stat<T: StatType<DataType: Clone + Send + Sync + 'static> + Send + Sync + 'static>(
    mut stats: Query<&mut Stat<T>>,
) {
    for mut stat in stats.iter_mut() {
        stat.clear();
    }
}

fn apply_add<T: StatType<DataType: Add + Clone + Send + Sync + 'static> + Send + Sync + 'static>(
    mut stats: Query<&mut Stat<T>>,
) {
    for mut stat in stats.iter_mut() {
        stat.running_total = stat
            .running_total
            .clone()
            .add(stat.running_op_total.clone());
        stat.running_op_total = Add::zero();
    }
}

fn apply_mul_before<
    T: StatType<DataType: Add + Mul + Clone + Send + Sync + 'static> + Send + Sync + 'static,
>(
    mut stats: Query<&mut Stat<T>>,
) {
    for mut stat in stats.iter_mut() {
        stat.running_total = stat
            .running_total
            .clone()
            .mul(T::DataType::one().add(stat.running_op_total.clone()));
        stat.running_op_total = Add::zero();
    }
}

fn apply_mul_after<
    T: StatType<DataType: Add + Mul + Clone + Send + Sync + 'static> + Send + Sync + 'static,
>(
    mut stats: Query<&mut Stat<T>>,
) {
    for mut stat in stats.iter_mut() {
        stat.running_total = stat
            .running_total
            .clone()
            .mul(T::DataType::one().add(stat.running_op_total.clone()));
        stat.running_op_total = Add::zero();
    }
}

/// Extension trait for [`App`] for stat registration methods.
pub trait AppExt {
    /// Register a [`StatType`].
    fn add_stat_type<T: StatType<DataType: Clone + Send + Sync + 'static> + Send + Sync + 'static>(
        &mut self,
    ) -> &mut Self;

    /// Register a [`StatModifierAdd`].
    fn add_stat_modifier_add<
        T: StatType<DataType: Add + Clone + Send + Sync + 'static> + Send + Sync + 'static,
        Modifier: StatModifierAdd<T> + Component,
    >(
        &mut self,
    ) -> &mut Self;

    /// Register a [`StatModifierMul`].
    fn add_stat_modifier_mul<
        T: StatType<DataType: Add + Mul + Clone + Send + Sync + 'static> + Send + Sync + 'static,
        Modifier: StatModifierMul<T> + Component,
    >(
        &mut self,
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
        T: StatType<DataType: Add + Clone + Send + Sync + 'static> + Send + Sync + 'static,
        Modifier: StatModifierAdd<T> + Component,
    >(
        &mut self,
    ) -> &mut Self {
        self.add_systems(
            PreUpdate,
            (
                (move |mut stats: Query<(&mut Stat<T>, &Modifier)>| {
                    for (mut stat, modifier) in stats.iter_mut() {
                        stat.running_op_total = stat.running_op_total.clone().add(modifier.add());
                    }
                })
                .in_set(StatsSystems::Op(DataTypeOp::Add)),
                apply_add::<T>.in_set(StatsSystems::Apply(DataTypeOp::Add)),
            ),
        );
        self
    }

    fn add_stat_modifier_mul<
        T: StatType<DataType: Add + Mul + Clone + Send + Sync + 'static> + Send + Sync + 'static,
        Modifier: StatModifierMul<T> + Component,
    >(
        &mut self,
    ) -> &mut Self {
        self.add_systems(
            PreUpdate,
            (
                (move |mut stats: Query<(&mut Stat<T>, &Modifier)>| {
                    for (mut stat, modifier) in stats.iter_mut() {
                        stat.running_op_total =
                            stat.running_op_total.clone().add(modifier.mul_before());
                    }
                })
                .in_set(StatsSystems::Op(DataTypeOp::MulBefore)),
                apply_mul_before::<T>.in_set(StatsSystems::Apply(DataTypeOp::MulBefore)),
                (move |mut stats: Query<(&mut Stat<T>, &Modifier)>| {
                    for (mut stat, modifier) in stats.iter_mut() {
                        stat.running_op_total =
                            stat.running_op_total.clone().add(modifier.mul_after());
                    }
                })
                .in_set(StatsSystems::Op(DataTypeOp::MulAfter)),
                apply_mul_after::<T>.in_set(StatsSystems::Apply(DataTypeOp::MulAfter)),
            ),
        );
        self
    }
}

/// Plugin required for stats to work.
///
/// Sets up [`StatsPlugin`].
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

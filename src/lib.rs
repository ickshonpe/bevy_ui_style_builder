use bevy::prelude::*;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use thiserror::Error;

pub mod prelude {
    pub use crate::node;
    pub use crate::style;
    pub use crate::Breadth;
    pub use crate::NodeColorExt;
    pub use crate::NumRect;
    pub use crate::StyleBuilderExt;
}

pub fn node() -> NodeBundle {
    NodeBundle::default()
}

pub fn style() -> Style {
    Style::default()
}

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

/// An enum that describes the possible evaluatable (numeric) values in a flexbox layout.
/// `Breadth` is used to represent distances from side to side that the UI layout algorithm
/// cannot infer automatically.
///
/// `Breadth` is similar to the `Val` enum except that it has no non-evaluatable variants
/// and its methods have been adapted to to reflect that they always have a defined output.
/// For example, [`Val::try_add_with_size`] can return an error, but `Breadth`'s equivalent
/// returns an `f32` and is renamed to [`Breadth::add_with_size`].
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Breadth {
    /// A value in pixels
    Px(f32),
    /// A value in percent
    Percent(f32),
}

impl Default for Breadth {
    fn default() -> Self {
        Self::Px(0.)
    }
}

impl From<Breadth> for Val {
    fn from(value: Breadth) -> Self {
        match value {
            Breadth::Px(inner) => Val::Px(inner),
            Breadth::Percent(inner) => Val::Percent(inner),
        }
    }
}

impl TryFrom<Val> for Breadth {
    type Error = BreadthConversionError;
    fn try_from(value: Val) -> Result<Self, Self::Error> {
        match value {
            Val::Px(inner) => Ok(Breadth::Px(inner)),
            Val::Percent(inner) => Ok(Breadth::Percent(inner)),
            _ => Err(Self::Error::NonEvaluateable),
        }
    }
}

impl Mul<f32> for Breadth {
    type Output = Breadth;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Breadth::Px(value) => Breadth::Px(value * rhs),
            Breadth::Percent(value) => Breadth::Percent(value * rhs),
        }
    }
}

impl MulAssign<f32> for Breadth {
    fn mul_assign(&mut self, rhs: f32) {
        match self {
            Breadth::Px(value) | Breadth::Percent(value) => *value *= rhs,
        }
    }
}

impl Div<f32> for Breadth {
    type Output = Breadth;

    fn div(self, rhs: f32) -> Self::Output {
        match self {
            Breadth::Px(value) => Breadth::Px(value / rhs),
            Breadth::Percent(value) => Breadth::Percent(value / rhs),
        }
    }
}

impl DivAssign<f32> for Breadth {
    fn div_assign(&mut self, rhs: f32) {
        match self {
            Breadth::Px(value) | Breadth::Percent(value) => *value /= rhs,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Error)]
pub enum BreadthArithmeticError {
    #[error("the variants of the Breadths don't match")]
    NonIdenticalVariants,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Error)]
pub enum BreadthConversionError {
    #[error("Cannot convert from non-evaluatable variants (non-numeric)")]
    NonEvaluateable,
}

impl Breadth {
    /// Tries to add the values of two [`Breadth`]s.
    /// Returns [`BreadthArithmeticError::NonIdenticalVariants`] if two [`Breadth`]s are of different variants.
    pub fn try_add(&self, rhs: Breadth) -> Result<Breadth, BreadthArithmeticError> {
        match (self, rhs) {
            (Breadth::Px(value), Breadth::Px(rhs_value)) => Ok(Breadth::Px(value + rhs_value)),
            (Breadth::Percent(value), Breadth::Percent(rhs_value)) => {
                Ok(Breadth::Percent(value + rhs_value))
            }
            _ => Err(BreadthArithmeticError::NonIdenticalVariants),
        }
    }

    /// Adds `rhs` to `self` and assigns the result to `self` (see [`Breadth::try_add`])
    pub fn try_add_assign(&mut self, rhs: Breadth) -> Result<(), BreadthArithmeticError> {
        *self = self.try_add(rhs)?;
        Ok(())
    }

    /// Tries to subtract the values of two [`Breadth`]s.
    /// Returns [`BreadthArithmeticError::NonIdenticalVariants`] if two [`Breadth`]s are of different variants.
    pub fn try_sub(&self, rhs: Breadth) -> Result<Breadth, BreadthArithmeticError> {
        match (self, rhs) {
            (Breadth::Px(value), Breadth::Px(rhs_value)) => Ok(Breadth::Px(value - rhs_value)),
            (Breadth::Percent(value), Breadth::Percent(rhs_value)) => {
                Ok(Breadth::Percent(value - rhs_value))
            }
            _ => Err(BreadthArithmeticError::NonIdenticalVariants),
        }
    }

    /// Subtracts `rhs` from `self` and assigns the result to `self` (see [`Breadth::try_sub`])
    pub fn try_sub_assign(&mut self, rhs: Breadth) -> Result<(), BreadthArithmeticError> {
        *self = self.try_sub(rhs)?;
        Ok(())
    }

    /// A convenience function for simple evaluation of [`Breadth::Percent`] variant into a concrete [`Breadth::Px`] value.
    /// Otherwise it returns an [`f32`] containing the evaluated value in pixels.
    ///
    /// **Note:** If a [`Breadth::Px`] is evaluated, it's inner value returned unchanged.
    pub fn evaluate(&self, size: f32) -> f32 {
        match self {
            Breadth::Percent(value) => size * value / 100.0,
            Breadth::Px(value) => *value,
        }
    }

    /// Similar to [`Breadth::try_add`], but performs [`Breadth::evaluate`] on both values before adding.
    /// Returns an [`f32`] value in pixels.
    pub fn add_with_size(&self, rhs: Breadth, size: f32) -> f32 {
        self.evaluate(size) + rhs.evaluate(size)
    }

    /// Similar to [`Breadth::try_add_assign`], but performs [`Breadth::evaluate`] on both values before adding.
    /// The value gets converted to [`Breadth::Px`].
    pub fn add_assign_with_size(&mut self, rhs: Breadth, size: f32) {
        *self = Breadth::Px(self.evaluate(size) + rhs.evaluate(size));
    }

    /// Similar to [`Breadth::try_sub`], but performs [`Breadth::evaluate`] on both values before subtracting.
    /// Returns an [`f32`] value in pixels.
    pub fn sub_with_size(&self, rhs: Breadth, size: f32) -> f32 {
        self.evaluate(size) - rhs.evaluate(size)
    }

    /// Similar to [`Breadth::try_sub_assign`], but performs [`Breadth::evaluate`] on both values before adding.
    /// The value gets converted to [`Breadth::Px`].
    pub fn sub_assign_with_size(&mut self, rhs: Breadth, size: f32) {
        *self = Breadth::Px(self.add_with_size(rhs, size));
    }
}

/// A copy of [`UiRect`] but without non-numeric values.
#[derive(Clone, Copy, Debug, Default)]
pub struct NumRect {
    pub left: Breadth,
    pub right: Breadth,
    pub top: Breadth,
    pub bottom: Breadth,
}

impl NumRect {
    pub fn new(left: Breadth, right: Breadth, top: Breadth, bottom: Breadth) -> Self {
        NumRect {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn all(value: Breadth) -> Self {
        NumRect {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

    pub fn horizontal(value: Breadth) -> Self {
        NumRect {
            left: value,
            right: value,
            ..Default::default()
        }
    }

    pub fn vertical(value: Breadth) -> Self {
        NumRect {
            top: value,
            bottom: value,
            ..Default::default()
        }
    }

    pub fn left(value: Breadth) -> Self {
        NumRect {
            left: value,
            ..Default::default()
        }
    }

    pub fn right(value: Breadth) -> Self {
        NumRect {
            right: value,
            ..Default::default()
        }
    }

    pub fn top(value: Breadth) -> Self {
        NumRect {
            top: value,
            ..Default::default()
        }
    }

    pub fn bottom(value: Breadth) -> Self {
        NumRect {
            bottom: value,
            ..Default::default()
        }
    }
}

impl From<NumRect> for UiRect {
    fn from(rect: NumRect) -> Self {
        UiRect {
            left: rect.left.into(),
            right: rect.right.into(),
            top: rect.top.into(),
            bottom: rect.bottom.into(),
        }
    }
}

impl From<Breadth> for Either<Breadth, NumRect> {
    fn from(breadth: Breadth) -> Self {
        Either::Left(breadth)
    }
}

impl From<NumRect> for Either<Breadth, NumRect> {
    fn from(rect: NumRect) -> Self {
        Either::Right(rect)
    }
}

impl From<Val> for Either<Val, UiRect> {
    fn from(val: Val) -> Self {
        Either::Left(val)
    }
}

impl From<UiRect> for Either<Val, UiRect> {
    fn from(rect: UiRect) -> Self {
        Either::Right(rect)
    }
}

pub trait StyleBuilderExt: Sized {
    fn update_style(self, s: impl FnOnce(&mut Style)) -> Self;

    /// Set the left displacement of the node.
    fn left(self, left: Val) -> Self {
        self.update_style(|style| {
            style.position.left = left;
        })
    }

    /// Set the right displacement of the node.
    fn right(self, right: Val) -> Self {
        self.update_style(|style| {
            style.position.right = right;
        })
    }

    /// Set the top displacement of the node.
    fn top(self, top: Val) -> Self {
        self.update_style(|style| {
            style.position.top = top;
        })
    }

    /// Set the bottom displacement of the node.
    fn bottom(self, bottom: Val) -> Self {
        self.update_style(|style| {
            style.position.bottom = bottom;
        })
    }

    /// Display this node and its children.
    fn display(self) -> Self {
        self.update_style(|style| {
            style.display = Display::Flex;
        })
    }

    /// Hide this node and its children.
    fn disable(self) -> Self {
        self.update_style(|style| {
            style.display = Display::None;
        })
    }

    /// Set the flex-direction to `Row`.
    fn row(self) -> Self {
        self.update_style(|style| {
            style.flex_direction = FlexDirection::Row;
        })
    }

    /// Set the flex-direction to `Column`.
    fn column(self) -> Self {
        self.update_style(|style| {
            style.flex_direction = FlexDirection::Column;
        })
    }

    /// Set the flex-direction to `RowReverse`.
    fn row_reverse(self) -> Self {
        self.update_style(|style| {
            style.flex_direction = FlexDirection::RowReverse;
        })
    }

    /// Set the flex-direction to `ColumnReverse`.
    fn column_reverse(self) -> Self {
        self.update_style(|style| {
            style.flex_direction = FlexDirection::ColumnReverse;
        })
    }

    /// No wrap.
    fn no_wrap(self) -> Self {
        self.update_style(|style| {
            style.flex_wrap = FlexWrap::NoWrap;
        })
    }

    /// Set flex-wrap to `Wrap`.
    fn wrap(self) -> Self {
        self.update_style(|style| {
            style.flex_wrap = FlexWrap::Wrap;
        })
    }

    /// Set flex-wrap to `WrapReverse`.
    fn wrap_reverse(self) -> Self {
        self.update_style(|style| {
            style.flex_wrap = FlexWrap::WrapReverse;
        })
    }

    /// Set the position type to absolute.
    fn absolute(self) -> Self {
        self.update_style(|style| {
            style.position_type = PositionType::Absolute;
        })
    }

    /// Set the position type to relative.
    fn relative(self) -> Self {
        self.update_style(|style| {
            style.position_type = PositionType::Relative;
        })
    }

    /// Set flex-basis.
    fn basis(self, basis: Val) -> Self {
        self.update_style(|style| {
            style.flex_basis = basis;
        })
    }

    /// Set flex-grow.
    fn grow(self, growth: f32) -> Self {
        self.update_style(|style| {
            style.flex_grow = growth;
        })
    }

    /// Set flex-shrink.
    fn shrink(self, shrink: f32) -> Self {
        self.update_style(|style| {
            style.flex_shrink = shrink;
        })
    }

    /// Set the minimum width of the node.
    fn min_width(self, min_width: Val) -> Self {
        self.update_style(|style| {
            style.min_size.width = min_width;
        })
    }

    /// Set the width of the node.
    fn width(self, width: Val) -> Self {
        self.update_style(|style| {
            style.size.width = width;
        })
    }

    /// Set the maximum width of the node.
    fn max_width(self, max_width: Val) -> Self {
        self.update_style(|style| {
            style.max_size.width = max_width;
        })
    }

    /// Set the minimum height of the node.
    fn min_height(self, min_height: Val) -> Self {
        self.update_style(|style| {
            style.min_size.height = min_height;
        })
    }

    /// Set the height of the node.
    fn height(self, height: Val) -> Self {
        self.update_style(|style| {
            style.size.height = height;
        })
    }

    /// Set the maximum height of the node.
    fn max_height(self, max_height: Val) -> Self {
        self.update_style(|style| {
            style.max_size.height = max_height;
        })
    }

    /// Set margins for the node.
    fn margin(self, margin: impl Into<Either<Val, UiRect>>) -> Self {
        self.update_style(|style| {
            style.margin = match margin.into() {
                Either::Left(val) => UiRect::all(val),
                Either::Right(rect) => rect,
            };
        })
    }

    /// Set border thickness for the node.
    fn border(self, border: impl Into<Either<Breadth, NumRect>>) -> Self {
        self.update_style(|style| {
            style.border = match border.into() {
                Either::Left(breadth) => NumRect::all(breadth),
                Either::Right(rect) => rect,
            }
            .into();
        })
    }

    /// Set padding for the node.
    fn padding(self, padding: impl Into<Either<Breadth, NumRect>>) -> Self {
        self.update_style(|style| {
            style.padding = match padding.into() {
                Either::Left(breadth) => NumRect::all(breadth),
                Either::Right(rect) => rect,
            }
            .into();
        })
    }

    /// Clip overflow.
    fn hide_overflow(self) -> Self {
        self.update_style(|style| {
            style.overflow = Overflow::Hidden;
        })
    }

    /// Show overflow.
    fn show_overflow(self) -> Self {
        self.update_style(|style| {
            style.overflow = Overflow::Visible;
        })
    }

    /// The minimum size of the node.
    /// `min_size` overrides the `size` and `max_size` properties.
    fn min_size(self, size: Size) -> Self {
        self.update_style(|style| {
            style.min_size = size;
        })
    }

    /// Set the size of the node.
    fn size(self, size: Size) -> Self {
        self.update_style(|style| {
            style.size = size;
        })
    }

    /// Set width and height to the same value.
    fn size_all(self, value: Val) -> Self {
        self.update_style(|style| {
            style.size = Size::new(value, value);
        })
    }

    /// The maximum size of the node.
    fn max_size(self, size: Size) -> Self {
        self.update_style(|style| {
            style.max_size = size;
        })
    }

    /// How this item is aligned according to the cross axis
    fn align_self(self, align: AlignSelf) -> Self {
        self.update_style(|style| {
            style.align_self = align;
        })
    }

    /// How child nodes are aligned according to the cross axis
    fn align_items(self, align: AlignItems) -> Self {
        self.update_style(|style| {
            style.align_items = align;
        })
    }

    /// Defines how wrapped lines are aligned within the flexbox.
    fn align_content(self, align: AlignContent) -> Self {
        self.update_style(|style| {
            style.align_content = align;
        })
    }

    /// How child nodes are aligned according to the main axis
    fn justify_content(self, justify: JustifyContent) -> Self {
        self.update_style(|style| {
            style.justify_content = justify;
        })
    }

    fn align_items_center(self) -> Self {
        self.update_style(|style| {
            style.align_items = AlignItems::Center;
        })
    }

    fn align_items_start(self) -> Self {
        self.update_style(|style| {
            style.align_items = AlignItems::FlexStart;
        })
    }

    fn align_items_end(self) -> Self {
        self.update_style(|style| {
            style.align_items = AlignItems::FlexEnd;
        })
    }

    fn align_items_stretch(self) -> Self {
        self.update_style(|style| {
            style.align_items = AlignItems::Stretch;
        })
    }

    fn align_items_baseline(self) -> Self {
        self.update_style(|style| {
            style.align_items = AlignItems::Baseline;
        })
    }

    fn align_self_auto(self) -> Self {
        self.update_style(|style| {
            style.align_self = AlignSelf::Auto;
        })
    }

    fn align_self_center(self) -> Self {
        self.update_style(|style| {
            style.align_self = AlignSelf::Center;
        })
    }

    fn align_self_start(self) -> Self {
        self.update_style(|style| {
            style.align_self = AlignSelf::FlexStart;
        })
    }

    fn align_self_end(self) -> Self {
        self.update_style(|style| {
            style.align_self = AlignSelf::FlexEnd;
        })
    }

    fn align_self_stretch(self) -> Self {
        self.update_style(|style| {
            style.align_self = AlignSelf::Stretch;
        })
    }

    fn align_self_baseline(self) -> Self {
        self.update_style(|style| {
            style.align_self = AlignSelf::Baseline;
        })
    }

    fn align_content_center(self) -> Self {
        self.update_style(|style| {
            style.align_content = AlignContent::Center;
        })
    }

    fn align_content_start(self) -> Self {
        self.update_style(|style| {
            style.align_content = AlignContent::FlexStart;
        })
    }

    fn align_content_end(self) -> Self {
        self.update_style(|style| {
            style.align_content = AlignContent::FlexEnd;
        })
    }

    fn align_content_space_between(self) -> Self {
        self.update_style(|style| {
            style.align_content = AlignContent::SpaceBetween;
        })
    }

    fn align_content_space_around(self) -> Self {
        self.update_style(|style| {
            style.align_content = AlignContent::SpaceAround;
        })
    }

    fn align_content_stretch(self) -> Self {
        self.update_style(|style| {
            style.align_content = AlignContent::Stretch;
        })
    }

    fn justify_content_center(self) -> Self {
        self.update_style(|style| {
            style.justify_content = JustifyContent::Center;
        })
    }

    fn justify_content_start(self) -> Self {
        self.update_style(|style| {
            style.justify_content = JustifyContent::FlexStart;
        })
    }

    fn justify_content_end(self) -> Self {
        self.update_style(|style| {
            style.justify_content = JustifyContent::FlexEnd;
        })
    }

    fn justify_content_space_between(self) -> Self {
        self.update_style(|style| {
            style.justify_content = JustifyContent::SpaceBetween;
        })
    }

    fn justify_content_space_around(self) -> Self {
        self.update_style(|style| {
            style.justify_content = JustifyContent::SpaceAround;
        })
    }

    fn justify_content_space_evenly(self) -> Self {
        self.update_style(|style| {
            style.justify_content = JustifyContent::SpaceEvenly;
        })
    }
}

impl StyleBuilderExt for NodeBundle {
    fn update_style(mut self, s: impl FnOnce(&mut Style)) -> Self {
        s(&mut self.style);
        self
    }
}

impl StyleBuilderExt for TextBundle {
    fn update_style(mut self, s: impl FnOnce(&mut Style)) -> Self {
        s(&mut self.style);
        self
    }
}

impl StyleBuilderExt for ImageBundle {
    fn update_style(mut self, s: impl FnOnce(&mut Style)) -> Self {
        s(&mut self.style);
        self
    }
}

impl StyleBuilderExt for Style {
    fn update_style(mut self, s: impl FnOnce(&mut Style)) -> Self {
        s(&mut self);
        self
    }
}

impl StyleBuilderExt for &mut Style {
    fn update_style(self, s: impl FnOnce(&mut Style)) -> Self {
        s(self);
        self
    }
}

pub trait NodeColorExt {
    fn background_color(self, color: Color) -> Self;
}

impl NodeColorExt for NodeBundle {
    fn background_color(mut self, color: Color) -> Self {
        self.background_color = color.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::BreadthArithmeticError;
    use bevy::prelude::*;

    #[test]
    fn test_breadth() {
        let inner = 10.;
        assert_eq!(Val::from(Breadth::Px(inner)), Val::Px(10.0));
        assert_eq!(Val::from(Breadth::Percent(inner)), Val::Percent(10.0));
    }

    #[test]
    fn breadth_try_add() {
        let px_sum = Breadth::Px(20.).try_add(Breadth::Px(22.)).unwrap();
        let percent_sum = Breadth::Percent(50.)
            .try_add(Breadth::Percent(50.))
            .unwrap();

        assert_eq!(px_sum, Breadth::Px(42.));
        assert_eq!(percent_sum, Breadth::Percent(100.));
    }

    #[test]
    fn breadth_try_add_to_self() {
        let mut breadth = Breadth::Px(5.);

        breadth.try_add_assign(Breadth::Px(3.)).unwrap();

        assert_eq!(breadth, Breadth::Px(8.));
    }

    #[test]
    fn breadth_try_sub() {
        let px_sum = Breadth::Px(72.).try_sub(Breadth::Px(30.)).unwrap();
        let percent_sum = Breadth::Percent(100.)
            .try_sub(Breadth::Percent(50.))
            .unwrap();

        assert_eq!(px_sum, Breadth::Px(42.));
        assert_eq!(percent_sum, Breadth::Percent(50.));
    }

    #[test]
    fn different_variant_breadth_try_add() {
        let different_variant_sum_1 = Breadth::Px(50.).try_add(Breadth::Percent(50.));
        let different_variant_sum_2 = Breadth::Percent(50.).try_add(Breadth::Px(50.));

        assert_eq!(
            different_variant_sum_1,
            Err(BreadthArithmeticError::NonIdenticalVariants)
        );
        assert_eq!(
            different_variant_sum_2,
            Err(BreadthArithmeticError::NonIdenticalVariants)
        );
    }

    #[test]
    fn different_variant_breadth_try_sub() {
        let different_variant_diff_1 = Breadth::Px(50.).try_sub(Breadth::Percent(50.));
        let different_variant_diff_2 = Breadth::Percent(50.).try_sub(Breadth::Px(50.));

        assert_eq!(
            different_variant_diff_1,
            Err(BreadthArithmeticError::NonIdenticalVariants)
        );
        assert_eq!(
            different_variant_diff_2,
            Err(BreadthArithmeticError::NonIdenticalVariants)
        );
    }

    #[test]
    fn breadth_evaluate_percent() {
        let size = 250.;
        let result = Breadth::Percent(80.).evaluate(size);

        assert_eq!(result, size * 0.8);
    }

    #[test]
    fn breadth_evaluate_px() {
        let size = 250.;
        let result = Breadth::Px(10.).evaluate(size);

        assert_eq!(result, 10.);
    }

    #[test]
    fn breadth_add_with_size() {
        let size = 250.;

        let px_sum = Breadth::Px(21.).add_with_size(Breadth::Px(21.), size);
        let percent_sum = Breadth::Percent(20.).add_with_size(Breadth::Percent(30.), size);
        let mixed_sum = Breadth::Px(20.).add_with_size(Breadth::Percent(30.), size);

        assert_eq!(px_sum, 42.);
        assert_eq!(percent_sum, 0.5 * size);
        assert_eq!(mixed_sum, 20. + 0.3 * size);
    }

    #[test]
    fn breadth_sub_with_size() {
        let size = 250.;

        let px_sum = Breadth::Px(60.).sub_with_size(Breadth::Px(18.), size);
        let percent_sum = Breadth::Percent(80.).sub_with_size(Breadth::Percent(30.), size);
        let mixed_sum = Breadth::Percent(50.).sub_with_size(Breadth::Px(30.), size);

        assert_eq!(px_sum, 42.);
        assert_eq!(percent_sum, 0.5 * size);
        assert_eq!(mixed_sum, 0.5 * size - 30.);
    }

    #[test]
    fn breadth_arithmetic_error_messages() {
        assert_eq!(
            format!("{}", BreadthArithmeticError::NonIdenticalVariants),
            "the variants of the Breadths don't match"
        );
    }

    #[test]
    fn from_breadth_to_val() {
        let inner_value = 11.;

        assert_eq!(Val::from(Breadth::Px(inner_value)), Val::Px(inner_value));
        assert_eq!(
            Val::from(Breadth::Percent(inner_value)),
            Val::Percent(inner_value)
        );
    }

    #[test]
    fn try_from_val_to_breadth() {
        let inner_value = 22.;

        assert_eq!(
            Breadth::try_from(Val::Auto),
            Err(crate::BreadthConversionError::NonEvaluateable)
        );
        assert_eq!(
            Breadth::try_from(Val::Px(inner_value)),
            Ok(Breadth::Px(inner_value))
        );
        assert_eq!(
            Breadth::try_from(Val::Percent(inner_value)),
            Ok(Breadth::Percent(inner_value))
        );
        assert_eq!(
            Breadth::try_from(Val::Undefined),
            Err(crate::BreadthConversionError::NonEvaluateable)
        );
    }

    #[test]
    fn node_bundle_left() {
        let value = Val::Px(1.);
        let node = node().left(value);
        assert_eq!(node.style.position.left, value);
    }
}

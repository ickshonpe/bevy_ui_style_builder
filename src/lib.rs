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
    pub use crate::StyleBuilderExt;
    pub use crate::NodeBundleBuilderExt;
    pub use crate::NumRect;
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

pub trait StyleWriterExt: Sized {
    fn style(self, s: impl FnOnce(&mut Style)) -> Self;

    /// Set the left displacement of the node.
    fn left(self, left: Val) -> Self {
        self.style(|style| { style.position.left = left; })
    }

    /// Set the right displacement of the node.
    fn right(self, right: Val) -> Self {
        self.style(|style| { style.position.right = right; })
    }

    /// Set the top displacement of the node.
    fn top(self, top: Val) -> Self {
        self.style(|style| { style.position.top = top; })
    }

    /// Set the bottom displacement of the node.
    fn bottom(self, bottom: Val) -> Self {
        self.style(|style| { style.position.bottom = bottom; })
    }

    /// Display this node and its children.
    fn display(self) -> Self {
        self.style(|style| { style.display = Display::Flex; })
    }

    /// Hide this node and its children.
    fn disable(self) -> Self {
        self.style(|style| { style.display = Display::None; })
    }

    /// Set the flex-direction to `Row`.
    fn row(self) -> Self {
        self.style(|style| { style.flex_direction = FlexDirection::Row; })
    }

    /// Set the flex-direction to `Column`.
    fn column(self) -> Self {
        self.style(|style| { style.flex_direction = FlexDirection::Column; })
    }

    /// Set the flex-direction to `RowReverse`.
    fn row_reverse(self) -> Self {
        self.style(|style| { style.flex_direction = FlexDirection::RowReverse; })
    }

    /// Set the flex-direction to `ColumnReverse`.
    fn column_reverse(self) -> Self {
        self.style(|style| { style.flex_direction = FlexDirection::ColumnReverse; })
    }

    /// No wrap.
    fn no_wrap(self) -> Self {
        self.style(|style| { style.flex_wrap = FlexWrap::NoWrap; })
    }

    /// Set flex-wrap to `Wrap`.
    fn wrap(self) -> Self {
        self.style(|style| { style.flex_wrap = FlexWrap::Wrap; })
    }

    /// Set flex-wrap to `WrapReverse`.
    fn wrap_reverse(self) -> Self {
        self.style(|style| { style.flex_wrap = FlexWrap::WrapReverse; })
    }
    
    /// Set the position type to absolute.
    fn absolute(self) -> Self {
        self.style(|style| { style.position_type = PositionType::Absolute; })
    }
    
    /// Set the position type to relative.
    fn relative(self) -> Self {
        self.style(|style| { style.position_type = PositionType::Relative; })
    }
    
    fn basis(self, basis: Val) -> Self {
        self.style(|style| { style.flex_basis = basis; })
    }
    
    fn grow(self, growth: f32) -> Self {
        self.style(|style| { style.flex_grow = growth; })
    }
    
    fn shrink(self, shrink: f32) -> Self {
        self.style(|style| { style.flex_shrink = shrink; })
    }

    fn min_width(self, min_width: Val) -> Self {
        self.style(|style| { style.min_size.width = min_width; })
    }
    
    fn width(self, width: Val) -> Self {
        self.style(|style| { style.size.width = width; })
    }
    
    fn max_width(self, max_width: Val) -> Self {
        self.style(|style| { style.max_size.width = max_width; })
    }

    fn min_height(self, min_height: Val) -> Self {
        self.style(|style| { style.min_size.height = min_height; })
    }

    fn height(self, height: Val) -> Self {
        self.style(|style| { style.size.height = height; })
    }

    fn max_height(self, max_height: Val) -> Self {
        self.style(|style| { style.max_size.height = max_height; })
    }

    fn margin(self, margin: impl Into<Either<Val, UiRect>>) -> Self {
        self.style(|style| { style.margin = match margin.into() {
            Either::Left(val) => UiRect::all(val),
            Either::Right(rect) => rect,
        }; })
    }

    fn border(self, border: impl Into<Either<Breadth, NumRect>>) -> Self {
        self.style(|style| { style.border = match border.into() {
            Either::Left(breadth) => NumRect::all(breadth),
            Either::Right(rect) => rect,
        }.into(); })
    }

    fn padding(self, padding: impl Into<Either<Breadth, NumRect>>) -> Self {
        self.style(|style| { style.padding = match padding.into() {
            Either::Left(breadth) => NumRect::all(breadth),
            Either::Right(rect) => rect,
        }.into(); })
    }

    fn hide_overflow(self) -> Self {
        self.style(|style| { style.overflow = Overflow::Hidden; })
    }

    fn show_overflow(self) -> Self {
        self.style(|style| { style.overflow = Overflow::Visible; })
    }

    fn min_size(self, size: Size) -> Self {
        self.style(|style| { style.min_size = size; })
    }

    fn size(self, size: Size) -> Self {
        self.style(|style| { style.size = size; })
    }

    fn max_size(self, size: Size) -> Self {
        self.style(|style| { style.max_size = size; })
    }

    fn align_self(self, align: AlignSelf) -> Self {
        self.style(|style| { style.align_self = align; })
    }

    fn align_items(self, align: AlignItems) -> Self {
        self.style(|style| { style.align_items = align; })
    }

    fn align_content(self, align: AlignContent) -> Self {
        self.style(|style| { style.align_content = align; })
    }

    fn justify_content(self, justify: JustifyContent) -> Self {
        self.style(|style| { style.justify_content = justify; })
    }
}

pub trait StyleBuilderExt {
    /// Set the left displacement of the node.
    fn left(self, value: Val) -> Self;
    
    /// Set the right displacement of the node.
    fn right(self, value: Val) -> Self;
    
    /// Set the top displacement of the node.
    fn top(self, value: Val) -> Self;
    
    /// Set the bottom displacement of the node.
    fn bottom(self, value: Val) -> Self;
    
    /// Display this node and its children.
    fn display(self) -> Self;
    
    /// Hide this node and its children.
    fn disable(self) -> Self;

    /// Set the flex-direction to row.
    fn row(self) -> Self;

    /// Set the flex-direction to column.
    fn column(self) -> Self;
    
    /// Set the flex-direction to row reverse.
    fn row_reverse(self) -> Self;
    
    /// Set the flex-direction to column reverse.
    fn column_reverse(self) -> Self;
    
    /// Set the flex-wrap to wrap.
    fn wrap(self) -> Self;
    
    /// Set the flex-wrap to wrap reverse.
    fn wrap_reverse(self) -> Self;
    
    /// Set the position type to absolute.
    fn absolute(self) -> Self;
    
    /// Set the position type to relative.
    fn relative(self) -> Self;
    
    /// Set flex-grow.
    fn grow(self, growth: f32) -> Self;
    
    /// Set flex-shrink.
    fn shrink(self, shrink: f32) -> Self;
    
    /// Set flex-basis.
    fn basis(self, basis: Val) -> Self;
    
    /// Set the minimum width of the node.
    fn min_width(self, min_width: Val) -> Self;
    
    /// Set the width of the node.
    fn width(self, width: Val) -> Self;
    
    /// Set the maximum width of the node.
    fn max_width(self, max_width: Val) -> Self;
    
    /// Set the minimum height of the node.
    fn min_height(self, min_height: Val) -> Self;    

    /// Set the height of the node.
    fn height(self, height: Val) -> Self;    
    
    /// Set the maximum height of the node.
    fn max_height(self, max_height: Val) -> Self;
    
    /// Set the margin of the node.
    fn margin(self, margin: impl Into<Either<Val, UiRect>>) -> Self;
    
    /// Set the thickness of the node's border.
    fn border(self, border: impl Into<Either<Breadth, NumRect>>) -> Self;
    
    /// Set the padding of the node.
    fn padding(self, padding: impl Into<Either<Breadth, NumRect>>) -> Self;
    
    /// Clip any overflow.
    fn hide_overflow(self) -> Self;
    
    /// Show any overflow.
    fn show_overflow(self) -> Self;
    
    /// The minimum size of the node.
    /// `min_size` overrides the `size` and `max_size` properties.
    fn min_size(self, size: Size) -> Self;
    
    /// The ideal size of the node.
    fn size(self, size: Size) -> Self;
    
    /// The maximum size of the node.
    /// `max_size overrides `size` and is overriden by `min_size`.
    fn max_size(self, size: Size) -> Self;

    /// How this item is aligned according to the cross axis
    fn align_self(self, align: AlignSelf) -> Self;

    /// How items are aligned according to the cross axis
    fn align_items(self, align: AlignItems) -> Self;

    /// When wrapping is enabled, defines how each line is aligned within the flexbox.
    fn align_content(self, align: AlignContent) -> Self;

    // How items are aligned along the main axis.
    fn justify_content(self, justify: JustifyContent) -> Self;    
}

pub trait NodeBuilder {
    fn make_node(self) -> NodeBundle;
}

impl NodeBuilder for Style {
    fn make_node(self) -> NodeBundle {
        NodeBundle {
            style: self,
            ..Default::default()
        }
    }
}

pub trait NodeBundleBuilderExt {
    fn from_style(style: Style) -> Self;
    fn color(self, color: Color) -> Self;
    fn style(self, s: impl FnOnce(&mut Style) -> &mut Style) -> Self;
}

impl NodeBundleBuilderExt for NodeBundle {
    fn from_style(style: Style) -> Self {
        NodeBundle {
            style,
            ..Default::default()
        }
    }

    fn color(mut self, color: Color) -> Self {
        self.background_color = color.into();
        self
    }

    fn style(mut self, s: impl FnOnce(&mut Style) -> &mut Style) -> Self {
        s(&mut self.style);
        self
    }

    
}

impl StyleBuilderExt for Style {
    /// Set left displacement of the node.
    fn left(mut self, value: Val) -> Self {
        self.position.left = value;
        self
    }

    /// Set right displacement of the node.
    fn right(mut self, value: Val) -> Self {
        self.position.right = value;
        self
    }

    /// Set top displacement of the node.
    fn top(mut self, value: Val) -> Self {
        self.position.top = value;
        self
    }

    /// Set bottom displacement of the node.
    fn bottom(mut self, value: Val) -> Self {
        self.position.bottom = value;
        self
    }

    
    fn display(mut self) -> Self {
        self.display = Display::Flex;
        self
    }

    fn disable(mut self) -> Self {
        self.display = Display::None;
        self
    }

    /// Set the flex-direction to row.
    fn row(mut self) -> Self {
        self.flex_direction = FlexDirection::Row;
        self
    }

    /// Set the flex-direction to column.
    fn column(mut self) -> Self {
        self.flex_direction = FlexDirection::Column;
        self
    }

    /// Set the flex-direction to row-reverse.
    fn row_reverse(mut self) -> Self {
        self.flex_direction = FlexDirection::RowReverse;
        self
    }

    /// Set the flex-direction to column-reverse.
    fn column_reverse(mut self) -> Self {
        self.flex_direction = FlexDirection::ColumnReverse;
        self
    }

    fn wrap(mut self) -> Self {
        self.flex_wrap = FlexWrap::Wrap;
        self
    }

    fn wrap_reverse(mut self) -> Self {
        self.flex_wrap = FlexWrap::WrapReverse;
        self
    }

    fn absolute(mut self) -> Self {
        self.position_type = PositionType::Absolute;
        self
    }

    fn relative(mut self) -> Self {
        self.position_type = PositionType::Relative;
        self
    }

    fn grow(mut self, growth: f32) -> Self {
        self.flex_grow = growth;
        self
    }

    fn shrink(mut self, shrink: f32) -> Self {
        self.flex_shrink = shrink;
        self
    }

    fn basis(mut self, basis: Val) -> Self {
        self.flex_basis = basis;
        self
    }

    fn min_width(mut self, min_width: Val) -> Self {
        self.min_size.width = min_width;
        self
    }

    fn width(mut self, width: Val) -> Self {
        self.size.width = width;
        self
    }

    fn max_width(mut self, max_width: Val) -> Self {
        self.max_size.width = max_width;
        self
    }

    fn min_height(mut self, min_height: Val) -> Self {
        self.min_size.height = min_height;
        self
    }

    fn height(mut self, height: Val) -> Self {
        self.size.height = height;
        self
    }

    fn max_height(mut self, max_height: Val) -> Self {
        self.max_size.height = max_height;
        self
    }

    fn margin(mut self, margin: impl Into<Either<Val, UiRect>>) -> Self {
        match margin.into() {
            Either::Left(value) => {
                self.margin = UiRect::all(value);
            }
            Either::Right(rect) => {
                self.margin = rect;
            }
        }
        self
    }

    fn border(mut self, border: impl Into<Either<Breadth, NumRect>>) -> Self {
        self.border = match border.into() {
                Either::Left(value) => NumRect::all(value),                
                Either::Right(rect) => rect,
            }.into();
        self
    }

    fn padding(mut self, padding: impl Into<Either<Breadth, NumRect>>) -> Self {
        self.padding =  match padding.into() {
            Either::Left(value) => NumRect::all(value),                
            Either::Right(rect) => rect,
        }.into();
        self
    }

    fn hide_overflow(mut self) -> Self {
        self.overflow = Overflow::Hidden;
        self
    }

    fn show_overflow(mut self) -> Self {
        self.overflow = Overflow::Visible;
        self
    }

    fn min_size(mut self, size: Size) -> Self {
        self.min_size = size;
        self
    }

    fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    fn max_size(mut self, size: Size) -> Self {
        self.max_size = size;
        self
    }

    fn align_self(mut self, align: AlignSelf) -> Self {
        self.align_self = align;
        self
    }

    fn align_items(mut self, align: AlignItems) -> Self {
        self.align_items = align;
        self
    }

    fn align_content(mut self, align: AlignContent) -> Self {
        self.align_content = align;
        self
    }

    fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.justify_content = justify;
        self
    }
}

impl StyleBuilderExt for &mut Style {
    
    fn left(self, value: Val) -> Self {
        self.position.left = value;
        self
    }

    /// Set right displacement of the node.
    fn right(self, value: Val) -> Self {
        self.position.right = value;
        self
    }

    /// Set top displacement of the node.
    fn top(self, value: Val) -> Self {
        self.position.top = value;
        self
    }

    /// Set bottom displacement of the node.
    fn bottom(self, value: Val) -> Self {
        self.position.bottom = value;
        self
    }

    /// Display this node and its children.
    fn display(self) -> Self {
        self.display = Display::Flex;
        self
    }

    /// Hide this node and its children.
    fn disable(self) -> Self {
        self.display = Display::None;
        self
    }

    /// Set the flex-direction to row.
    fn row(self) -> Self {
        self.flex_direction = FlexDirection::Row;
        self
    }

    /// Set the flex-direction to column.
    fn column(self) -> Self {
        self.flex_direction = FlexDirection::Column;
        self
    }

    /// Set the flex-direction to row reverse.
    fn row_reverse(self) -> Self {
        self.flex_direction = FlexDirection::RowReverse;
        self
    }

    /// Set the flex-direction to column reverse.
    fn column_reverse(self) -> Self {
        self.flex_direction = FlexDirection::ColumnReverse;
        self
    }

    /// set flex wrap to wrap
    fn wrap(self) -> Self {
        self.flex_wrap = FlexWrap::Wrap;
        self
    }

    /// Set flex wrap to wrap reverse
    fn wrap_reverse(self) -> Self {
        self.flex_wrap = FlexWrap::WrapReverse;
        self
    }

    /// Use absolute positioning.
    fn absolute(self) -> Self {
        self.position_type = PositionType::Absolute;
        self
    }

    /// Use relative positioning.
    fn relative(self) -> Self {
        self.position_type = PositionType::Relative;
        self
    }

    /// Set flex-grow.
    fn grow(self, growth: f32) -> Self {
        self.flex_grow = growth;
        self
    }

    /// Set flex-shrink.
    fn shrink(self, shrink: f32) -> Self {
        self.flex_shrink = shrink;
        self
    }

    /// Set flex-basis.
    fn basis(self, basis: Val) -> Self {
        self.flex_basis = basis;
        self
    }

    /// Set the minimum width of the node.
    fn min_width(self, min_width: Val) -> Self {
        self.min_size.width = min_width;
        self
    }

    /// Set the width of the node.
    fn width(self, width: Val) -> Self {
        self.size.width = width;
        self
    }

    /// Set the maximum width of the node.
    fn max_width(self, max_width: Val) -> Self {
        self.max_size.width = max_width;
        self
    }

    /// Set the minimum height of the node.
    fn min_height(self, min_height: Val) -> Self {
        self.min_size.height = min_height;
        self
    }

    /// Set the height of the node.
    fn height(self, height: Val) -> Self {
        self.size.height = height;
        self
    }

    /// Set the maximum height of the node.
    fn max_height(self, max_height: Val) -> Self {
        self.max_size.height = max_height;
        self
    }

    /// Set the margin of the node.
    fn margin(self, margin: impl Into<Either<Val, UiRect>>) -> Self {
        match margin.into() {
            Either::Left(value) => {
                self.margin = UiRect::all(value);
            }
            Either::Right(rect) => {
                self.margin = rect;
            }
        }
        self
    }

    /// Set the thickness of the node's border.
    fn border(self, border: impl Into<Either<Breadth, NumRect>>) -> Self {
        self.border = match border.into() {
                Either::Left(value) => NumRect::all(value),                
                Either::Right(rect) => rect,
            }.into();
        self
    }

    /// Set the padding of the node.
    fn padding(self, padding: impl Into<Either<Breadth, NumRect>>) -> Self {
        self.padding =  match padding.into() {
            Either::Left(value) => NumRect::all(value),                
            Either::Right(rect) => rect,
        }.into();
        self
    }

    /// Clip any overflow.
    fn hide_overflow(self) -> Self {
        self.overflow = Overflow::Hidden;
        self
    }

    /// Show any overflow.
    fn show_overflow(self) -> Self {
        self.overflow = Overflow::Visible;
        self
    }

    /// The minimum size of the node.
    /// `min_size` overrides the `size` and `max_size` properties.
    fn min_size(self, size: Size) -> Self {
        self.min_size = size;
        self
    }

    /// The ideal size of the node.
    fn size(self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// The maximum size of the node.
    /// `max_size overrides `size` and is overriden by `min_size`.
    fn max_size(self, size: Size) -> Self {
        self.max_size = size;
        self
    }

    /// How this item is aligned according to the cross axis
    fn align_self(self, align: AlignSelf) -> Self {
        self.align_self = align;
        self
    }

    /// How items are aligned according to the cross axis
    fn align_items(self, align: AlignItems) -> Self {
        self.align_items = align;
        self
    }

    /// When wrapping is enabled, defines how each line is aligned within the flexbox.
    fn align_content(self, align: AlignContent) -> Self {
        self.align_content = align;
        self
    }

    // How items are aligned along the main axis.
    fn justify_content(self, justify: JustifyContent) -> Self {
        self.justify_content = justify;
        self
    }
}

impl StyleBuilderExt for NodeBundle {
    fn left(self, left: Val) -> Self {
        self.style(|style| style.left(left))
    }

    fn right(mut self, right: Val) -> Self {
        (&mut self.style).right(right);
        self
    }

    fn top(mut self, top: Val) -> Self {
        (&mut self.style).top(top);
        self
    }

    fn bottom(mut self, bottom: Val) -> Self {
        (&mut self.style).bottom(bottom);
        self
    }

    fn display(mut self) -> Self {
        (&mut self.style).display();
        self
    }

    fn disable(mut self) -> Self {
        (&mut self.style).disable();
        self
    }

    fn row(mut self) -> Self {
        (&mut self.style).row();
        self
    }

    fn column(mut self) -> Self {
        (&mut self.style).column();
        self
    }

    fn row_reverse(mut self) -> Self {
        (&mut self.style).row_reverse();
        self
    }

    fn column_reverse(mut self) -> Self {
        (&mut self.style).column_reverse();
        self
    }

    fn wrap(mut self) -> Self {
        (&mut self.style).wrap();
        self
    }

    fn wrap_reverse(mut self) -> Self {
        (&mut self.style).wrap_reverse();
        self
    }

    fn min_width(mut self, min_width: Val) -> Self {
        (&mut self.style).min_width(min_width);
        self
    }

    fn absolute(mut self) -> Self {
        (&mut self.style).absolute();
        self
    }

    fn relative(mut self) -> Self {
        (&mut self.style).relative();
        self
    }

    fn basis(mut self, basis: Val) -> Self {
        (&mut self.style).basis(basis);
        self
    }

    fn grow(mut self, growth: f32) -> Self {
        (&mut self.style).grow(growth);
        self
    }

    fn shrink(mut self, shrink: f32) -> Self {
        (&mut self.style).shrink(shrink);
        self
    }

    fn width(mut self, width: Val) -> Self {
        (&mut self.style).width(width);
        self
    }

    fn max_width(mut self, max_width: Val) -> Self {
        (&mut self.style).max_width(max_width);
        self
    }

    fn min_height(mut self, min_height: Val) -> Self {
        (&mut self.style).min_height(min_height);
        self
    }

    fn height(mut self, height: Val) -> Self {
        (&mut self.style).height(height);
        self
    }

    fn max_height(mut self, max_height: Val) -> Self {
        (&mut self.style).max_height(max_height);
        self
    }

    fn margin(mut self, margin: impl Into<Either<Val, UiRect>>) -> Self {
        (&mut self.style).margin(margin);
        self
    }

    fn border(mut self, border: impl Into<Either<Breadth, NumRect>>) -> Self {
        (&mut self.style).border(border);
        self
    }

    fn padding(mut self, padding: impl Into<Either<Breadth, NumRect>>) -> Self {
        (&mut self.style).padding(padding);
        self
    }

    fn hide_overflow(mut self) -> Self {
        (&mut self.style).hide_overflow();
        self
    }

    fn show_overflow(mut self) -> Self {
        (&mut self.style).show_overflow();
        self
    }

    fn min_size(mut self, size: Size) -> Self {
        (&mut self.style).min_size(size);
        self
    }

    fn size(mut self, size: Size) -> Self {
        (&mut self.style).size(size);
        self
    }

    fn max_size(mut self, size: Size) -> Self {
        (&mut self.style).max_size(size);
        self
    }

    fn align_self(mut self, align: AlignSelf) -> Self {
        (&mut self.style).align_self(align);
        self
    }

    fn align_items(mut self, align: AlignItems) -> Self {
        (&mut self.style).align_items(align);
        self
    }

    fn align_content(mut self, align: AlignContent) -> Self {
        (&mut self.style).align_content(align);
        self
    }

    fn justify_content(mut self, justify: JustifyContent) -> Self {
        (&mut self.style).justify_content(justify);
        self
    }
}




#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use crate::BreadthArithmeticError;
    use crate::prelude::*;

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
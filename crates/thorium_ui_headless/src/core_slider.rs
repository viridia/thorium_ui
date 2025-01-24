use bevy::prelude::*;

use crate::{InteractionDisabled, ValueChange};

/// A headless slider widget, which can be used to build custom sliders. This component emits
/// [`ValueChange`] events when the slider value changes. Note that the value in the event is
/// unclamped - the reason is that the receiver may want to quantize or otherwise modify the value
/// before clamping. It is the receiver's responsibility to update the slider's value when
/// the value change event is received.
#[derive(Component, Debug)]
#[require(DragState)]
pub struct CoreSlider {
    pub value: f32,
    pub min: f32,
    pub max: f32,
}

impl CoreSlider {
    /// Constructg a new [`CoreSlider`].
    pub fn new(value: f32, min: f32, max: f32) -> Self {
        Self { value, min, max }
    }

    /// Get the current value of the slider.
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Set the value of the slider, clamping it to the min and max values.
    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(self.min, self.max);
    }

    /// Set the minimum and maximum value of the slider, clamping the current value to the new
    /// range.
    pub fn set_range(&mut self, min: f32, max: f32) {
        self.min = min;
        self.max = max;
        self.value = self.value.clamp(min, max).clamp(0., 1.);
    }

    /// Compute the position of the thumb on the slider, as a value between 0 and 1.
    pub fn thumb_position(&self) -> f32 {
        if self.max > self.min {
            (self.value - self.min) / (self.max - self.min)
        } else {
            0.5
        }
    }
}

/// Component used to manage the state of a slider during dragging.
#[derive(Component, Default)]
pub struct DragState {
    /// Whether the slider is currently being dragged.
    dragging: bool,
    /// The value of the slider when dragging started.
    offset: f32,
}

pub(crate) fn slider_on_drag_start(
    mut trigger: Trigger<Pointer<DragStart>>,
    mut q_state: Query<(&CoreSlider, &mut DragState, Has<InteractionDisabled>)>,
) {
    if let Ok((slider, mut drag, disabled)) = q_state.get_mut(trigger.target()) {
        trigger.propagate(false);
        if !disabled {
            drag.dragging = true;
            drag.offset = slider.value;
        }
    }
}

pub(crate) fn slider_on_drag(
    mut trigger: Trigger<Pointer<Drag>>,
    mut q_state: Query<(&ComputedNode, &CoreSlider, &mut DragState)>,
    mut commands: Commands,
) {
    if let Ok((node, slider, drag)) = q_state.get_mut(trigger.target()) {
        trigger.propagate(false);
        if drag.dragging {
            let distance = trigger.event().distance;
            // Measure node width and slider value.
            let slider_width = node.size().x * node.inverse_scale_factor;
            let range = slider.max - slider.min;
            let new_value = if range > 0. {
                drag.offset + (distance.x * range) / slider_width
            } else {
                slider.min + range * 0.5
            };
            commands.trigger_targets(ValueChange(new_value), trigger.target());
        }
    }
}

pub(crate) fn slider_on_drag_end(
    mut trigger: Trigger<Pointer<DragEnd>>,
    mut q_state: Query<(&CoreSlider, &mut DragState)>,
) {
    if let Ok((_slider, mut drag)) = q_state.get_mut(trigger.target()) {
        trigger.propagate(false);
        if drag.dragging {
            drag.dragging = false;
        }
    }
}

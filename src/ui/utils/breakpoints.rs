use std::collections::BTreeMap;

use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::ui::{common::UiComponentId, screens::ScreenComponentsRegistry};

pub enum BreakpointsDirection {
    Horizontal,
    Vertical,
}

pub type SectionSizes = Vec<u16>;

/// Small utility for making Screens' layout responsive.
#[derive(Debug)]
pub struct Breakpoints {
    label: String,
    default_breakpoints: SectionSizes,
    sections_breakpoints: BTreeMap<u16, SectionSizes>,
}

impl Breakpoints {
    pub fn new<S: Into<String>>(label: S, default_breakpoints: &[u16]) -> Self {
        assert!(
            Self::check_section_sizes(default_breakpoints),
            "Breakpoints with label '{}': incorrect percentage sum for default size",
            label.into(),
        );
        Self {
            label: label.into(),
            default_breakpoints: default_breakpoints.into(),
            sections_breakpoints: BTreeMap::new(),
        }
    }

    pub fn breakpoint(mut self, size: u16, breakpoints: &[u16]) -> Self {
        assert!(
            breakpoints.len() == self.default_breakpoints.len(),
            "Breakpoints with label '{}': size mismatch",
            self.label
        );
        assert!(
            Self::check_section_sizes(breakpoints),
            "Breakpoints with label '{}': incorrect percentage sum for size '{}'",
            self.label,
            size
        );
        self.sections_breakpoints.insert(size, breakpoints.to_vec());
        self
    }

    pub fn to_constraints(&self, size: u16) -> Vec<Constraint> {
        self.to_percentages(size)
            .iter()
            .map(|percentage| Constraint::Percentage(*percentage))
            .collect()
    }

    /// Apply the layout according to the configured breakpoints. To be used from a Screen's layout computing function.
    pub fn apply(
        &self,
        components_registry: &mut ScreenComponentsRegistry,
        components_ids: &[UiComponentId],
        frame_size: Rect,
        along: BreakpointsDirection,
    ) {
        use BreakpointsDirection::*;

        let size = match along {
            Horizontal => frame_size.width,
            Vertical => frame_size.height,
        };
        let percentages = self.to_percentages(size);
        assert!(
            percentages.len() == components_ids.len(),
            "Breakpoints with label '{}': incorrect components length in 'apply' method",
            self.label
        );

        let direction = match along {
            Horizontal => Direction::Horizontal,
            Vertical => Direction::Vertical,
        };
        let layout = Layout::default().margin(2).direction(direction);

        let mut constraints = Vec::with_capacity(percentages.len());
        let mut active_components_ids = Vec::with_capacity(percentages.len());
        for (i, percentage) in percentages.iter().enumerate() {
            if *percentage == 0 {
                continue;
            }
            constraints.push(Constraint::Percentage(*percentage));
            active_components_ids.push(components_ids[i]);
        }

        let chunks = layout.constraints(constraints).split(frame_size);
        for (i, component_id) in active_components_ids.iter().enumerate() {
            components_registry.insert(*component_id, chunks[i]);
        }
    }

    fn to_percentages(&self, size: u16) -> Vec<u16> {
        let breakpoint_size =
            self.sections_breakpoints
                .keys()
                .fold(0, |acc, &bp| if bp <= size { bp } else { acc });
        self.sections_breakpoints
            .get(&breakpoint_size)
            .unwrap_or(&self.default_breakpoints)
            .to_vec()
    }

    fn check_section_sizes(sizes: &[u16]) -> bool {
        sizes.iter().sum::<u16>() == 100
    }
}

#[cfg(test)]
mod tests {
    use tui::layout::{Constraint, Rect};

    use crate::ui::{screens::ScreenComponentsRegistry, utils::breakpoints::Breakpoints};

    use super::BreakpointsDirection;

    #[test]
    #[should_panic]
    fn test_breakpoints_size_check() {
        Breakpoints::new("test", &[50, 50]).breakpoint(60, &[10, 80, 10]);
    }

    #[test]
    #[should_panic]
    fn test_breakpoints_default_sizes_percentages_check() {
        Breakpoints::new("test", &[10, 30, 30]);
    }

    #[test]
    #[should_panic]
    fn test_breakpoints_percentages_check() {
        Breakpoints::new("test", &[10, 60, 30]).breakpoint(50, &[20, 30, 20]);
    }

    #[test]
    fn test_breakpoints_constraints() {
        fn sizes_to_constraints(sizes: &[u16]) -> Vec<Constraint> {
            sizes
                .iter()
                .map(|&size| Constraint::Percentage(size))
                .collect()
        }

        let breakpoints = Breakpoints::new("test", &[3, 90, 7])
            .breakpoint(50, &[7, 85, 8])
            .breakpoint(60, &[10, 80, 10]);
        assert_eq!(
            breakpoints.to_constraints(30),
            sizes_to_constraints(&[3, 90, 7])
        );
        assert_eq!(
            breakpoints.to_constraints(49),
            sizes_to_constraints(&[3, 90, 7])
        );
        assert_eq!(
            breakpoints.to_constraints(50),
            sizes_to_constraints(&[7, 85, 8]),
        );
        assert_eq!(
            breakpoints.to_constraints(60),
            sizes_to_constraints(&[10, 80, 10]),
        );
    }

    #[test]
    fn test_breakpoints_apply() {
        let mut components_registry = ScreenComponentsRegistry::new();
        let (component_id_a, component_id_b) = ("a", "b");
        let breakpoints = Breakpoints::new("test", &[10, 90]).breakpoint(30, &[0, 100]);

        breakpoints.apply(
            &mut components_registry,
            &[component_id_a, component_id_b],
            Rect::new(0, 0, 30, 25),
            BreakpointsDirection::Vertical,
        );
        assert!(components_registry.contains_key(&component_id_a));
        assert!(components_registry.contains_key(&component_id_b));

        components_registry.clear();
        breakpoints.apply(
            &mut components_registry,
            &[component_id_a, component_id_b],
            Rect::new(0, 0, 30, 35),
            BreakpointsDirection::Vertical,
        );
        assert!(!components_registry.contains_key(&component_id_a));
        assert!(components_registry.contains_key(&component_id_b));
    }
}

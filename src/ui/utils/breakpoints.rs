use std::collections::BTreeMap;

use tui::layout::Constraint;

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
        let breakpoint_size =
            self.sections_breakpoints
                .keys()
                .fold(0, |acc, &bp| if bp <= size { bp } else { acc });
        let sizes = self
            .sections_breakpoints
            .get(&breakpoint_size)
            .unwrap_or(&self.default_breakpoints);
        sizes
            .iter()
            .map(|&size| Constraint::Percentage(size))
            .collect()
    }

    fn check_section_sizes(sizes: &[u16]) -> bool {
        sizes.iter().fold(0, |acc, size| acc + size) == 100
    }
}

#[cfg(test)]
mod tests {
    use tui::layout::Constraint;

    use crate::ui::utils::breakpoints::Breakpoints;

    #[test]
    #[should_panic]
    fn test_breakpoint_size_check() {
        Breakpoints::new("test", &[50, 50]).breakpoint(60, &[10, 80, 10]);
    }

    #[test]
    #[should_panic]
    fn test_breakpoint_default_sizes_percentages_check() {
        Breakpoints::new("test", &[10, 30, 30]);
    }

    #[test]
    #[should_panic]
    fn test_breakpoint_percentages_check() {
        Breakpoints::new("test", &[10, 60, 30]).breakpoint(50, &[20, 30, 20]);
    }

    #[test]
    fn test_breakpoint_constraints() {
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
}

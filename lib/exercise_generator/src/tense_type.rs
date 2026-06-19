// Tense type classification for vocabulary tense entries (libD, pure Rust).
//
// A `tense:` line in the markdown vocabulary data carries a label such as
// `negative-present-informal (nai-form)`. The parenthetical qualifier (e.g.
// `(nai-form)`, `(ta-form)`) is stripped before matching against the fixed
// set of known tense labels — it is not semantically meaningless, it is just
// excluded from the enum variant name. See `.claude/specs/extended-vocab.md`.

/// Classifies a vocabulary tense label into a known fixed variant, or
/// preserves an unrecognized label verbatim via `Other`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TenseType {
    Present,
    PastFormal,
    PastInformal,
    NegativePresentFormal,
    NegativePresentInformal,
    NegativePastFormal,
    NegativePastInformal,
    PotentialFormFormal,
    PotentialFormInformal,
    PotentialNegativeForm,
    PotentialNegativeFormInformal,
    TeForm,
    Other(String),
}

impl TenseType {
    /// Classifies a raw tense label (as it appears after the `tense:` key in
    /// the markdown data) into a `TenseType`.
    ///
    /// A trailing parenthetical qualifier (e.g. `" (nai-form)"`) is stripped
    /// from the label before matching against the fixed variants — matching
    /// is case-sensitive against the canonical kebab-case label. If nothing
    /// matches, the original, unstripped label is preserved in
    /// `TenseType::Other` so no information is lost.
    pub fn from_label(label: &str) -> TenseType {
        let stripped = match label.find('(') {
            Some(paren_idx) => label[..paren_idx].trim_end(),
            None => label,
        };

        match stripped {
            "present" => TenseType::Present,
            "past-formal" => TenseType::PastFormal,
            "past-informal" => TenseType::PastInformal,
            "negative-present-formal" => TenseType::NegativePresentFormal,
            "negative-present-informal" => TenseType::NegativePresentInformal,
            "negative-past-formal" => TenseType::NegativePastFormal,
            "negative-past-informal" => TenseType::NegativePastInformal,
            "potential-form-formal" => TenseType::PotentialFormFormal,
            "potential-form-informal" => TenseType::PotentialFormInformal,
            "potential-negative-form" => TenseType::PotentialNegativeForm,
            "potential-negative-form-informal" => TenseType::PotentialNegativeFormInformal,
            "te-form" => TenseType::TeForm,
            _ => TenseType::Other(label.to_string()),
        }
    }

    /// Returns the canonical kebab-case label for a fixed variant, or the
    /// stored original label for `Other`.
    pub fn display_label(&self) -> String {
        match self {
            TenseType::Present => "present".to_string(),
            TenseType::PastFormal => "past-formal".to_string(),
            TenseType::PastInformal => "past-informal".to_string(),
            TenseType::NegativePresentFormal => "negative-present-formal".to_string(),
            TenseType::NegativePresentInformal => "negative-present-informal".to_string(),
            TenseType::NegativePastFormal => "negative-past-formal".to_string(),
            TenseType::NegativePastInformal => "negative-past-informal".to_string(),
            TenseType::PotentialFormFormal => "potential-form-formal".to_string(),
            TenseType::PotentialFormInformal => "potential-form-informal".to_string(),
            TenseType::PotentialNegativeForm => "potential-negative-form".to_string(),
            TenseType::PotentialNegativeFormInformal => {
                "potential-negative-form-informal".to_string()
            }
            TenseType::TeForm => "te-form".to_string(),
            TenseType::Other(label) => label.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_variants_match_their_plain_label() {
        assert_eq!(TenseType::from_label("present"), TenseType::Present);
        assert_eq!(TenseType::from_label("past-formal"), TenseType::PastFormal);
        assert_eq!(
            TenseType::from_label("past-informal"),
            TenseType::PastInformal
        );
        assert_eq!(
            TenseType::from_label("negative-present-formal"),
            TenseType::NegativePresentFormal
        );
        assert_eq!(
            TenseType::from_label("negative-present-informal"),
            TenseType::NegativePresentInformal
        );
        assert_eq!(
            TenseType::from_label("negative-past-formal"),
            TenseType::NegativePastFormal
        );
        assert_eq!(
            TenseType::from_label("negative-past-informal"),
            TenseType::NegativePastInformal
        );
        assert_eq!(
            TenseType::from_label("potential-form-formal"),
            TenseType::PotentialFormFormal
        );
        assert_eq!(
            TenseType::from_label("potential-form-informal"),
            TenseType::PotentialFormInformal
        );
        assert_eq!(
            TenseType::from_label("potential-negative-form"),
            TenseType::PotentialNegativeForm
        );
        assert_eq!(
            TenseType::from_label("potential-negative-form-informal"),
            TenseType::PotentialNegativeFormInformal
        );
        assert_eq!(TenseType::from_label("te-form"), TenseType::TeForm);
    }

    #[test]
    fn label_with_parenthetical_qualifier_still_matches_fixed_variant() {
        assert_eq!(
            TenseType::from_label("negative-present-informal (nai-form)"),
            TenseType::NegativePresentInformal
        );
        assert_eq!(
            TenseType::from_label("past-informal (ta-form)"),
            TenseType::PastInformal
        );
    }

    #[test]
    fn unrecognized_label_falls_back_to_other_with_original_text() {
        assert_eq!(
            TenseType::from_label("some-future-tense"),
            TenseType::Other("some-future-tense".to_string())
        );
    }

    #[test]
    fn display_label_round_trips_for_fixed_variant() {
        let tense = TenseType::from_label("potential-form-formal");
        assert_eq!(tense.display_label(), "potential-form-formal");
    }

    #[test]
    fn display_label_round_trips_for_other_variant() {
        let tense = TenseType::from_label("some-future-tense");
        assert_eq!(tense.display_label(), "some-future-tense");
    }
}

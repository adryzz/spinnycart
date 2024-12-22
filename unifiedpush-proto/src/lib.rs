/// Notification urgency, as defined in [RFC8030 Section 5.3](https://datatracker.ietf.org/doc/html/rfc8030#section-5.3)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Urgency {
    /// For very low urgency notifications, such as
    VeryLow = 0,
    Low = 1,
    #[default]
    Normal = 2,
    High = 3,
}

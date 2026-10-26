use kernel::dns_resolution::domain::value_objects::dns_resolution_window::DnsResolutionWindow;

pub struct DnsResolutionWindowMother;

#[allow(dead_code)]
impl DnsResolutionWindowMother {
    /// A window from unix seconds.
    pub fn create(first_seen: u64, last_seen: u64) -> DnsResolutionWindow {
        DnsResolutionWindow::from_unix_seconds(first_seen, Some(last_seen))
            .expect("DnsResolutionWindowMother::create received an invalid window")
    }

    /// A one-day window somewhere in 2020 to 2025.
    pub fn random() -> DnsResolutionWindow {
        let first_seen = 1_577_836_800 + rand::random::<u64>() % 157_766_400;
        Self::create(first_seen, first_seen + 86_400)
    }
}

use core::fmt;

pub enum LinkSpeed {
    NetDeviceSpeed10Half,
    NetDeviceSpeed10Full,
    NetDeviceSpeed100Half,
    NetDeviceSpeed100Full,
    NetDeviceSpeed1000Half,
    NetDeviceSpeed1000Full,
    NetDeviceSpeedUnknown,
}

impl fmt::Display for LinkSpeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinkSpeed::NetDeviceSpeed10Half => write!(f, "10BASE-T half duplex"),
            LinkSpeed::NetDeviceSpeed10Full => write!(f, "10BASE-T full duplex"),
            LinkSpeed::NetDeviceSpeed100Half => write!(f, "100BASE-TX half duplex"),
            LinkSpeed::NetDeviceSpeed100Full => write!(f, "100BASE-TX full duplex"),
            LinkSpeed::NetDeviceSpeed1000Half => write!(f, "1000BASE-T half duplex"),
            LinkSpeed::NetDeviceSpeed1000Full => write!(f, "1000BASE-T full duplex"),
            LinkSpeed::NetDeviceSpeedUnknown => write!(f, "Speed Unknown"),
        }
    }
}

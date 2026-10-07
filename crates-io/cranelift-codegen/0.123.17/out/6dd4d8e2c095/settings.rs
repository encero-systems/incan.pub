#[derive(Clone, PartialEq, Hash)] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:426
/// Flags group `shared`.
pub struct Flags {
    bytes: [u8; 12], // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:429
}
impl Flags {
    /// Create flags shared settings group.
    #[allow(unused_variables, reason = "generated code")] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:26
    pub fn new(builder: Builder) -> Self {
        let bvec = builder.state_for("shared"); // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:31
        let mut shared = Self { bytes: [0; 12] }; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:32
        debug_assert_eq!(bvec.len(), 12); // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:38
        shared.bytes[0..12].copy_from_slice(&bvec); // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:43
        shared // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:64
    }
}
impl Flags {
    /// Iterates the setting values.
    pub fn iter(&self) -> impl Iterator<Item = Value> + use<> {
        let mut bytes = [0; 12]; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:74
        bytes.copy_from_slice(&self.bytes[0..12]); // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:75
        DESCRIPTORS.iter().filter_map(move |d| {
            let values = match &d.detail {
                detail::Detail::Preset => return None, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:78
                detail::Detail::Enum { last, enumerators } => Some(TEMPLATE.enums(*last, *enumerators)), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:79
                _ => None // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:80
            }
            ; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:82
            Some(Value { name: d.name, detail: d.detail, values, value: bytes[d.offset as usize] }) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:83
        }
        ) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:85
    }
}
/// Values for `shared.regalloc_algorithm`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:147
pub enum RegallocAlgorithm {
    /// `backtracking`.
    Backtracking, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
}
impl RegallocAlgorithm {
    /// Returns a slice with all possible [RegallocAlgorithm] values. // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:92
    pub fn all() -> &'static [RegallocAlgorithm] {
        &[ // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:98
            Self::Backtracking, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
        ] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:104
    }
}
impl fmt::Display for RegallocAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Self::Backtracking => "backtracking", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
        }
        ) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:119
    }
}
impl core::str::FromStr for RegallocAlgorithm {
    type Err = (); // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:125
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "backtracking" => Ok(Self::Backtracking), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            _ => Err(()), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:131
        }
    }
}
/// Values for `shared.opt_level`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:147
pub enum OptLevel {
    /// `none`.
    None, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
    /// `speed`.
    Speed, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
    /// `speed_and_size`.
    SpeedAndSize, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
}
impl OptLevel {
    /// Returns a slice with all possible [OptLevel] values. // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:92
    pub fn all() -> &'static [OptLevel] {
        &[ // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:98
            Self::None, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
            Self::Speed, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
            Self::SpeedAndSize, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
        ] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:104
    }
}
impl fmt::Display for OptLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Self::None => "none", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
            Self::Speed => "speed", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
            Self::SpeedAndSize => "speed_and_size", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
        }
        ) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:119
    }
}
impl core::str::FromStr for OptLevel {
    type Err = (); // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:125
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            "speed" => Ok(Self::Speed), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            "speed_and_size" => Ok(Self::SpeedAndSize), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            _ => Err(()), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:131
        }
    }
}
/// Values for `shared.tls_model`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:147
pub enum TlsModel {
    /// `none`.
    None, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
    /// `elf_gd`.
    ElfGd, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
    /// `macho`.
    Macho, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
    /// `coff`.
    Coff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
}
impl TlsModel {
    /// Returns a slice with all possible [TlsModel] values. // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:92
    pub fn all() -> &'static [TlsModel] {
        &[ // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:98
            Self::None, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
            Self::ElfGd, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
            Self::Macho, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
            Self::Coff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
        ] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:104
    }
}
impl fmt::Display for TlsModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Self::None => "none", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
            Self::ElfGd => "elf_gd", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
            Self::Macho => "macho", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
            Self::Coff => "coff", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
        }
        ) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:119
    }
}
impl core::str::FromStr for TlsModel {
    type Err = (); // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:125
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            "elf_gd" => Ok(Self::ElfGd), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            "macho" => Ok(Self::Macho), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            "coff" => Ok(Self::Coff), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            _ => Err(()), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:131
        }
    }
}
/// Values for `shared.stack_switch_model`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:147
pub enum StackSwitchModel {
    /// `none`.
    None, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
    /// `basic`.
    Basic, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
    /// `update_windows_tib`.
    UpdateWindowsTib, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
}
impl StackSwitchModel {
    /// Returns a slice with all possible [StackSwitchModel] values. // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:92
    pub fn all() -> &'static [StackSwitchModel] {
        &[ // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:98
            Self::None, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
            Self::Basic, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
            Self::UpdateWindowsTib, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
        ] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:104
    }
}
impl fmt::Display for StackSwitchModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Self::None => "none", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
            Self::Basic => "basic", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
            Self::UpdateWindowsTib => "update_windows_tib", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
        }
        ) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:119
    }
}
impl core::str::FromStr for StackSwitchModel {
    type Err = (); // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:125
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            "basic" => Ok(Self::Basic), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            "update_windows_tib" => Ok(Self::UpdateWindowsTib), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            _ => Err(()), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:131
        }
    }
}
/// Values for `shared.libcall_call_conv`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:147
pub enum LibcallCallConv {
    /// `isa_default`.
    IsaDefault, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
    /// `fast`.
    Fast, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
    /// `cold`.
    Cold, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
    /// `system_v`.
    SystemV, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
    /// `windows_fastcall`.
    WindowsFastcall, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
    /// `apple_aarch64`.
    AppleAarch64, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
    /// `probestack`.
    Probestack, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
}
impl LibcallCallConv {
    /// Returns a slice with all possible [LibcallCallConv] values. // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:92
    pub fn all() -> &'static [LibcallCallConv] {
        &[ // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:98
            Self::IsaDefault, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
            Self::Fast, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
            Self::Cold, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
            Self::SystemV, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
            Self::WindowsFastcall, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
            Self::AppleAarch64, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
            Self::Probestack, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
        ] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:104
    }
}
impl fmt::Display for LibcallCallConv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Self::IsaDefault => "isa_default", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
            Self::Fast => "fast", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
            Self::Cold => "cold", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
            Self::SystemV => "system_v", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
            Self::WindowsFastcall => "windows_fastcall", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
            Self::AppleAarch64 => "apple_aarch64", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
            Self::Probestack => "probestack", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
        }
        ) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:119
    }
}
impl core::str::FromStr for LibcallCallConv {
    type Err = (); // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:125
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "isa_default" => Ok(Self::IsaDefault), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            "fast" => Ok(Self::Fast), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            "cold" => Ok(Self::Cold), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            "system_v" => Ok(Self::SystemV), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            "windows_fastcall" => Ok(Self::WindowsFastcall), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            "apple_aarch64" => Ok(Self::AppleAarch64), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            "probestack" => Ok(Self::Probestack), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            _ => Err(()), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:131
        }
    }
}
/// Values for `shared.probestack_strategy`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:147
pub enum ProbestackStrategy {
    /// `outline`.
    Outline, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
    /// `inline`.
    Inline, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
}
impl ProbestackStrategy {
    /// Returns a slice with all possible [ProbestackStrategy] values. // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:92
    pub fn all() -> &'static [ProbestackStrategy] {
        &[ // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:98
            Self::Outline, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
            Self::Inline, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
        ] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:104
    }
}
impl fmt::Display for ProbestackStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Self::Outline => "outline", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
            Self::Inline => "inline", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
        }
        ) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:119
    }
}
impl core::str::FromStr for ProbestackStrategy {
    type Err = (); // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:125
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "outline" => Ok(Self::Outline), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            "inline" => Ok(Self::Inline), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            _ => Err(()), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:131
        }
    }
}
/// User-defined settings.
#[allow(dead_code, reason = "generated code")] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:209
impl Flags {
    /// Dynamic numbered predicate getter.
    fn numbered_predicate(&self, p: usize) -> bool {
        self.bytes[9 + p / 8] & (1 << (p % 8)) != 0 // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:214
    }
    /// Algorithm to use in register allocator.
    ///
    /// Supported options:
    ///
    /// - `backtracking`: A backtracking allocator with range splitting; more expensive
    ///                   but generates better code.
    ///
    /// Note that the `single_pass` option is currently disabled because it does not
    /// have adequate support for the kinds of allocations required by exception
    /// handling (https://github.com/bytecodealliance/regalloc2/issues/217).
    pub fn regalloc_algorithm(&self) -> RegallocAlgorithm {
        match self.bytes[0] {
            0 => {
                RegallocAlgorithm::Backtracking
            }
            _ => {
                panic!("Invalid enum value")
            }
        }
    }
    /// Optimization level for generated code.
    ///
    /// Supported levels:
    ///
    /// - `none`: Minimise compile time by disabling most optimizations.
    /// - `speed`: Generate the fastest possible code
    /// - `speed_and_size`: like "speed", but also perform transformations aimed at reducing code size.
    pub fn opt_level(&self) -> OptLevel {
        match self.bytes[1] {
            0 => {
                OptLevel::None
            }
            1 => {
                OptLevel::Speed
            }
            2 => {
                OptLevel::SpeedAndSize
            }
            _ => {
                panic!("Invalid enum value")
            }
        }
    }
    /// Defines the model used to perform TLS accesses.
    pub fn tls_model(&self) -> TlsModel {
        match self.bytes[2] {
            3 => {
                TlsModel::Coff
            }
            1 => {
                TlsModel::ElfGd
            }
            2 => {
                TlsModel::Macho
            }
            0 => {
                TlsModel::None
            }
            _ => {
                panic!("Invalid enum value")
            }
        }
    }
    /// Defines the model used to performing stack switching.
    ///
    /// This determines the compilation of `stack_switch` instructions. If
    /// set to `basic`, we simply save all registers, update stack pointer
    /// and frame pointer (if needed), and jump to the target IP.
    /// If set to `update_windows_tib`, we *additionally* update information
    /// about the active stack in Windows' Thread Information Block.
    pub fn stack_switch_model(&self) -> StackSwitchModel {
        match self.bytes[3] {
            1 => {
                StackSwitchModel::Basic
            }
            0 => {
                StackSwitchModel::None
            }
            2 => {
                StackSwitchModel::UpdateWindowsTib
            }
            _ => {
                panic!("Invalid enum value")
            }
        }
    }
    /// Defines the calling convention to use for LibCalls call expansion.
    ///
    /// This may be different from the ISA default calling convention.
    ///
    /// The default value is to use the same calling convention as the ISA
    /// default calling convention.
    ///
    /// This list should be kept in sync with the list of calling
    /// conventions available in isa/call_conv.rs.
    pub fn libcall_call_conv(&self) -> LibcallCallConv {
        match self.bytes[4] {
            5 => {
                LibcallCallConv::AppleAarch64
            }
            2 => {
                LibcallCallConv::Cold
            }
            1 => {
                LibcallCallConv::Fast
            }
            0 => {
                LibcallCallConv::IsaDefault
            }
            6 => {
                LibcallCallConv::Probestack
            }
            3 => {
                LibcallCallConv::SystemV
            }
            4 => {
                LibcallCallConv::WindowsFastcall
            }
            _ => {
                panic!("Invalid enum value")
            }
        }
    }
    /// The log2 of the size of the stack guard region.
    ///
    /// Stack frames larger than this size will have stack overflow checked
    /// by calling the probestack function.
    ///
    /// The default is 12, which translates to a size of 4096.
    pub fn probestack_size_log2(&self) -> u8 {
        self.bytes[5] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:190
    }
    /// Controls what kinds of stack probes are emitted.
    ///
    /// Supported strategies:
    ///
    /// - `outline`: Always emits stack probes as calls to a probe stack function.
    /// - `inline`: Always emits inline stack probes.
    pub fn probestack_strategy(&self) -> ProbestackStrategy {
        match self.bytes[6] {
            1 => {
                ProbestackStrategy::Inline
            }
            0 => {
                ProbestackStrategy::Outline
            }
            _ => {
                panic!("Invalid enum value")
            }
        }
    }
    /// The log2 of the size to insert dummy padding between basic blocks
    ///
    /// This is a debugging option for stressing various cases during code
    /// generation without requiring large functions. This will insert
    /// 0-byte padding between basic blocks of the specified size.
    ///
    /// The amount of padding inserted two raised to the power of this value
    /// minus one. If this value is 0 then no padding is inserted.
    ///
    /// The default for this option is 0 to insert no padding as it's only
    /// intended for testing and development.
    pub fn bb_padding_log2_minus_one(&self) -> u8 {
        self.bytes[7] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:190
    }
    /// The log2 of the minimum alignment of functions
    /// The bigger of this value and the default alignment will be used as actual alignment.
    pub fn log2_min_function_alignment(&self) -> u8 {
        self.bytes[8] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:190
    }
    /// Enable the symbolic checker for register allocation.
    ///
    /// This performs a verification that the register allocator preserves
    /// equivalent dataflow with respect to the original (pre-regalloc)
    /// program. This analysis is somewhat expensive. However, if it succeeds,
    /// it provides independent evidence (by a carefully-reviewed, from-first-principles
    /// analysis) that no regalloc bugs were triggered for the particular compilations
    /// performed. This is a valuable assurance to have as regalloc bugs can be
    /// very dangerous and difficult to debug.
    pub fn regalloc_checker(&self) -> bool {
        self.numbered_predicate(0) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Enable verbose debug logs for regalloc2.
    ///
    /// This adds extra logging for regalloc2 output, that is quite valuable to understand
    /// decisions taken by the register allocator as well as debugging it. It is disabled by
    /// default, as it can cause many log calls which can slow down compilation by a large
    /// amount.
    pub fn regalloc_verbose_logs(&self) -> bool {
        self.numbered_predicate(1) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Do redundant-load optimizations with alias analysis.
    ///
    /// This enables the use of a simple alias analysis to optimize away redundant loads.
    /// Only effective when `opt_level` is `speed` or `speed_and_size`.
    pub fn enable_alias_analysis(&self) -> bool {
        self.numbered_predicate(2) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Run the Cranelift IR verifier at strategic times during compilation.
    ///
    /// This makes compilation slower but catches many bugs. The verifier is always enabled by
    /// default, which is useful during development.
    pub fn enable_verifier(&self) -> bool {
        self.numbered_predicate(3) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Enable proof-carrying code translation validation.
    ///
    /// This adds a proof-carrying-code mode. Proof-carrying code (PCC) is a strategy to verify
    /// that the compiler preserves certain properties or invariants in the compiled code.
    /// For example, a frontend that translates WebAssembly to CLIF can embed PCC facts in
    /// the CLIF, and Cranelift will verify that the final machine code satisfies the stated
    /// facts at each intermediate computed value. Loads and stores can be marked as "checked"
    /// and their memory effects can be verified as safe.
    pub fn enable_pcc(&self) -> bool {
        self.numbered_predicate(4) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Enable Position-Independent Code generation.
    pub fn is_pic(&self) -> bool {
        self.numbered_predicate(5) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Use colocated libcalls.
    ///
    /// Generate code that assumes that libcalls can be declared "colocated",
    /// meaning they will be defined along with the current function, such that
    /// they can use more efficient addressing.
    pub fn use_colocated_libcalls(&self) -> bool {
        self.numbered_predicate(6) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Enable the use of floating-point instructions.
    ///
    /// Disabling use of floating-point instructions is not yet implemented.
    pub fn enable_float(&self) -> bool {
        self.numbered_predicate(7) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Enable NaN canonicalization.
    ///
    /// This replaces NaNs with a single canonical value, for users requiring
    /// entirely deterministic WebAssembly computation. This is not required
    /// by the WebAssembly spec, so it is not enabled by default.
    pub fn enable_nan_canonicalization(&self) -> bool {
        self.numbered_predicate(8) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Enable the use of the pinned register.
    ///
    /// This register is excluded from register allocation, and is completely under the control of
    /// the end-user. It is possible to read it via the get_pinned_reg instruction, and to set it
    /// with the set_pinned_reg instruction.
    pub fn enable_pinned_reg(&self) -> bool {
        self.numbered_predicate(9) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Enable the use of atomic instructions
    pub fn enable_atomics(&self) -> bool {
        self.numbered_predicate(10) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Enable safepoint instruction insertions.
    ///
    /// This will allow the emit_stack_maps() function to insert the safepoint
    /// instruction on top of calls and interrupt traps in order to display the
    /// live reference values at that point in the program.
    pub fn enable_safepoints(&self) -> bool {
        self.numbered_predicate(11) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Enable various ABI extensions defined by LLVM's behavior.
    ///
    /// In some cases, LLVM's implementation of an ABI (calling convention)
    /// goes beyond a standard and supports additional argument types or
    /// behavior. This option instructs Cranelift codegen to follow LLVM's
    /// behavior where applicable.
    ///
    /// Currently, this applies only to Windows Fastcall on x86-64, and
    /// allows an `i128` argument to be spread across two 64-bit integer
    /// registers. The Fastcall implementation otherwise does not support
    /// `i128` arguments, and will panic if they are present and this
    /// option is not set.
    pub fn enable_llvm_abi_extensions(&self) -> bool {
        self.numbered_predicate(12) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Enable support for sret arg introduction when there are too many ret vals.
    ///
    /// When there are more returns than available return registers, the
    /// return value has to be returned through the introduction of a
    /// return area pointer. Normally this return area pointer has to be
    /// introduced as `ArgumentPurpose::StructReturn` parameter, but for
    /// backward compatibility reasons Cranelift also supports implicitly
    /// introducing this parameter and writing the return values through it.
    ///
    /// **This option currently does not conform to platform ABIs and the
    /// used ABI should not be assumed to remain the same between Cranelift
    /// versions.**
    ///
    /// This option is **deprecated** and will be removed in the future.
    ///
    /// Because of the above issues, and complexities of native ABI support
    /// for the concept in general, Cranelift's support for multiple return
    /// values may also be removed in the future (#9510). For the most
    /// robust solution, it is recommended to build a convention on top of
    /// Cranelift's primitives for passing multiple return values, for
    /// example by allocating a stackslot in the caller, passing it as an
    /// explicit StructReturn argument, storing return values in the callee,
    /// and loading results in the caller.
    pub fn enable_multi_ret_implicit_sret(&self) -> bool {
        self.numbered_predicate(13) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Generate unwind information.
    ///
    /// This increases metadata size and compile time, but allows for the
    /// debugger to trace frames, is needed for GC tracing that relies on
    /// libunwind (such as in Wasmtime), and is unconditionally needed on
    /// certain platforms (such as Windows) that must always be able to unwind.
    pub fn unwind_info(&self) -> bool {
        self.numbered_predicate(14) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Preserve frame pointers
    ///
    /// Preserving frame pointers -- even inside leaf functions -- makes it
    /// easy to capture the stack of a running program, without requiring any
    /// side tables or metadata (like `.eh_frame` sections). Many sampling
    /// profilers and similar tools walk frame pointers to capture stacks.
    /// Enabling this option will play nice with those tools.
    pub fn preserve_frame_pointers(&self) -> bool {
        self.numbered_predicate(15) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Generate CFG metadata for machine code.
    ///
    /// This increases metadata size and compile time, but allows for the
    /// embedder to more easily post-process or analyze the generated
    /// machine code. It provides code offsets for the start of each
    /// basic block in the generated machine code, and a list of CFG
    /// edges (with blocks identified by start offsets) between them.
    /// This is useful for, e.g., machine-code analyses that verify certain
    /// properties of the generated code.
    pub fn machine_code_cfg_info(&self) -> bool {
        self.numbered_predicate(16) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Enable the use of stack probes for supported calling conventions.
    pub fn enable_probestack(&self) -> bool {
        self.numbered_predicate(17) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Enable the use of jump tables in generated machine code.
    pub fn enable_jump_tables(&self) -> bool {
        self.numbered_predicate(18) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Enable Spectre mitigation on heap bounds checks.
    ///
    /// This is a no-op for any heap that needs no bounds checks; e.g.,
    /// if the limit is static and the guard region is large enough that
    /// the index cannot reach past it.
    ///
    /// This option is enabled by default because it is highly
    /// recommended for secure sandboxing. The embedder should consider
    /// the security implications carefully before disabling this option.
    pub fn enable_heap_access_spectre_mitigation(&self) -> bool {
        self.numbered_predicate(19) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Enable Spectre mitigation on table bounds checks.
    ///
    /// This option uses a conditional move to ensure that when a table
    /// access index is bounds-checked and a conditional branch is used
    /// for the out-of-bounds case, a misspeculation of that conditional
    /// branch (falsely predicted in-bounds) will select an in-bounds
    /// index to load on the speculative path.
    ///
    /// This option is enabled by default because it is highly
    /// recommended for secure sandboxing. The embedder should consider
    /// the security implications carefully before disabling this option.
    pub fn enable_table_access_spectre_mitigation(&self) -> bool {
        self.numbered_predicate(20) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Enable additional checks for debugging the incremental compilation cache.
    ///
    /// Enables additional checks that are useful during development of the incremental
    /// compilation cache. This should be mostly useful for Cranelift hackers, as well as for
    /// helping to debug false incremental cache positives for embedders.
    ///
    /// This option is disabled by default and requires enabling the "incremental-cache" Cargo
    /// feature in cranelift-codegen.
    pub fn enable_incremental_compilation_cache_checks(&self) -> bool {
        self.numbered_predicate(21) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
}
static DESCRIPTORS: [detail::Descriptor; 31] = [ // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:253
    detail::Descriptor {
        name: "regalloc_algorithm", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Algorithm to use in register allocator.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 0, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Enum { last: 0, enumerators: 0 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:274
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "opt_level", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Optimization level for generated code.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 1, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Enum { last: 2, enumerators: 1 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:274
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "tls_model", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Defines the model used to perform TLS accesses.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 2, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Enum { last: 3, enumerators: 4 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:274
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "stack_switch_model", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Defines the model used to performing stack switching.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 3, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Enum { last: 2, enumerators: 8 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:274
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "libcall_call_conv", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Defines the calling convention to use for LibCalls call expansion.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 4, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Enum { last: 6, enumerators: 11 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:274
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "probestack_size_log2", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "The log2 of the size of the stack guard region.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 5, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Num, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:282
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "probestack_strategy", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Controls what kinds of stack probes are emitted.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 6, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Enum { last: 1, enumerators: 18 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:274
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "bb_padding_log2_minus_one", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "The log2 of the size to insert dummy padding between basic blocks", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 7, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Num, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:282
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "log2_min_function_alignment", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "The log2 of the minimum alignment of functions", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 8, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Num, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:282
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "regalloc_checker", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Enable the symbolic checker for register allocation.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 9, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 0 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "regalloc_verbose_logs", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Enable verbose debug logs for regalloc2.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 9, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 1 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "enable_alias_analysis", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Do redundant-load optimizations with alias analysis.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 9, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 2 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "enable_verifier", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Run the Cranelift IR verifier at strategic times during compilation.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 9, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 3 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "enable_pcc", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Enable proof-carrying code translation validation.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 9, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 4 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "is_pic", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Enable Position-Independent Code generation.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 9, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 5 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "use_colocated_libcalls", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Use colocated libcalls.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 9, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 6 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "enable_float", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Enable the use of floating-point instructions.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 9, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 7 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "enable_nan_canonicalization", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Enable NaN canonicalization.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 10, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 0 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "enable_pinned_reg", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Enable the use of the pinned register.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 10, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 1 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "enable_atomics", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Enable the use of atomic instructions", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 10, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 2 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "enable_safepoints", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Enable safepoint instruction insertions.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 10, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 3 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "enable_llvm_abi_extensions", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Enable various ABI extensions defined by LLVM's behavior.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 10, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 4 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "enable_multi_ret_implicit_sret", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Enable support for sret arg introduction when there are too many ret vals.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 10, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 5 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "unwind_info", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Generate unwind information.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 10, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 6 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "preserve_frame_pointers", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Preserve frame pointers", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 10, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 7 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "machine_code_cfg_info", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Generate CFG metadata for machine code.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 11, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 0 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "enable_probestack", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Enable the use of stack probes for supported calling conventions.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 11, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 1 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "enable_jump_tables", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Enable the use of jump tables in generated machine code.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 11, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 2 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "enable_heap_access_spectre_mitigation", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Enable Spectre mitigation on heap bounds checks.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 11, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 3 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "enable_table_access_spectre_mitigation", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Enable Spectre mitigation on table bounds checks.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 11, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 4 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "enable_incremental_compilation_cache_checks", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Enable additional checks for debugging the incremental compilation cache.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 11, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 5 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
]; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:304
static ENUMERATORS: [&str; 20] = [ // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:307
    "backtracking", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
    "none", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
    "speed", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
    "speed_and_size", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
    "none", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
    "elf_gd", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
    "macho", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
    "coff", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
    "none", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
    "basic", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
    "update_windows_tib", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
    "isa_default", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
    "fast", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
    "cold", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
    "system_v", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
    "windows_fastcall", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
    "apple_aarch64", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
    "probestack", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
    "outline", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
    "inline", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
]; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:313
static HASH_TABLE: [u16; 64] = [ // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:323
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    2, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    11, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    29, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    13, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    24, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    30, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    20, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    18, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    7, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    6, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    1, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    0, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    3, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    14, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    22, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    26, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    8, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    17, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    12, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    19, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    9, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    21, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    23, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    5, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    28, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    25, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    27, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    10, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    15, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    4, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    16, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
]; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:339
static PRESETS: [(u8, u8); 0] = [ // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:342
]; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:359
static TEMPLATE: detail::Template = detail::Template {
    name: "shared", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:374
    descriptors: &DESCRIPTORS, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:375
    enumerators: &ENUMERATORS, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:376
    hash_table: &HASH_TABLE, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:377
    defaults: &[0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x8c, 0x44, 0x1c], // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:378
    presets: &PRESETS, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:379
}
; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:382
/// Create a `settings::Builder` for the shared settings group.
pub fn builder() -> Builder {
    Builder::new(&TEMPLATE) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:389
}
impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[shared]")?; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:398
        for d in &DESCRIPTORS {
            if !d.detail.is_preset() {
                write!(f, "{} = ", d.name)?; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:401
                TEMPLATE.format_toml_value(d.detail, self.bytes[d.offset as usize], f)?; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:402
                writeln!(f)?; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:406
            }
        }
        Ok(()) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:409
    }
}
impl Flags {
    /// Get the flag values as raw bytes for hashing.
    pub fn hash_key(&self) -> &[u8] {
        &self.bytes // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:419
    }
}

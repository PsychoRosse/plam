use std::num::NonZero;

#[cfg(feature = "clama")]
use clama::Event;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};

#[cfg_attr(feature = "clama", derive(Event))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WindowSize {
    pub(crate) width: NonZero<u32>,
    pub(crate) height: NonZero<u32>,
}

impl WindowSize {
    pub const MAX_DIMENSION: u32 = 16_384;
    pub const MAX_AREA: u64 = 67_108_864;
}

impl Default for WindowSize {
    #[inline]
    fn default() -> Self {
        Self::hd()
    }
}

impl WindowSize {
    #[inline]
    #[must_use]
    pub fn clamped(width: u32, height: u32) -> Self {
        let width = width.clamp(1, Self::MAX_DIMENSION);
        let height = height.clamp(1, Self::MAX_DIMENSION);

        let area = width as u64 * height as u64;

        let (mut width, mut height) = if area > Self::MAX_AREA {
            let scale = (Self::MAX_AREA as f64 / area as f64).sqrt();

            (
                ((width as f64 * scale).floor() as u32).max(1),
                ((height as f64 * scale).floor() as u32).max(1),
            )
        } else {
            (width, height)
        };

        while width as u64 * height as u64 > Self::MAX_AREA {
            if width >= height && width > 1 {
                width -= 1;
            } else if height > 1 {
                height -= 1;
            } else {
                break;
            }
        }

        Self {
            width: NonZero::<u32>::new(width).unwrap(),
            height: NonZero::<u32>::new(height).unwrap(),
        }
    }
}

impl WindowSize {
    #[inline]
    #[must_use]
    pub fn minimum() -> Self {
        Self::clamped(1, 1)
    }

    #[inline]
    #[must_use]
    pub fn hd() -> Self {
        Self::clamped(1280, 720)
    }

    #[inline]
    #[must_use]
    pub fn fhd() -> Self {
        Self::clamped(1920, 1080)
    }

    #[inline]
    #[must_use]
    pub fn qhd() -> Self {
        Self::clamped(2560, 1440)
    }

    #[inline]
    #[must_use]
    pub fn uhd() -> Self {
        Self::clamped(3840, 2160)
    }
}

impl WindowSize {
    #[inline]
    #[must_use]
    pub fn with_width(self, width: u32) -> Self {
        Self::clamped(width, self.height.get())
    }

    #[inline]
    #[must_use]
    pub fn with_height(self, height: u32) -> Self {
        Self::clamped(self.width.get(), height)
    }

    #[inline]
    #[must_use]
    pub const fn width(self) -> u32 {
        self.width.get()
    }

    #[inline]
    #[must_use]
    pub const fn height(self) -> u32 {
        self.height.get()
    }

    #[inline]
    #[must_use]
    pub const fn area(self) -> u64 {
        self.width.get() as u64 * self.height.get() as u64
    }

    #[inline]
    #[must_use]
    pub const fn is_landscape(self) -> bool {
        self.width.get() > self.height.get()
    }

    #[inline]
    #[must_use]
    pub const fn is_portrait(self) -> bool {
        self.height.get() > self.width.get()
    }

    #[inline]
    #[must_use]
    pub const fn is_square(self) -> bool {
        self.width.get() == self.height.get()
    }

    #[inline]
    #[must_use]
    pub const fn min_dimension(self) -> u32 {
        if self.width.get() < self.height.get() {
            self.width.get()
        } else {
            self.height.get()
        }
    }

    #[inline]
    #[must_use]
    pub const fn max_dimension(self) -> u32 {
        if self.width.get() > self.height.get() {
            self.width.get()
        } else {
            self.height.get()
        }
    }
}

impl From<[u32; 2]> for WindowSize {
    #[inline]
    fn from([width, height]: [u32; 2]) -> Self {
        Self::clamped(width, height)
    }
}

impl From<WindowSize> for [u32; 2] {
    #[inline]
    fn from(size: WindowSize) -> Self {
        [size.width(), size.height()]
    }
}

impl From<(u32, u32)> for WindowSize {
    #[inline]
    fn from((width, height): (u32, u32)) -> Self {
        Self::clamped(width, height)
    }
}

impl From<WindowSize> for (u32, u32) {
    #[inline]
    fn from(size: WindowSize) -> Self {
        (size.width(), size.height())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for WindowSize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawWindowSize {
            width: u32,
            height: u32,
        }

        let raw = RawWindowSize::deserialize(deserializer)?;

        Ok(Self::clamped(raw.width, raw.height))
    }
}

#[cfg(feature = "serde")]
impl Serialize for WindowSize {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("WindowSize", 2)?;

        state.serialize_field("width", &self.width())?;
        state.serialize_field("height", &self.height())?;

        state.end()
    }
}

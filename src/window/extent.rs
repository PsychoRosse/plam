use std::num::NonZeroU32;

#[cfg(feature = "clama")]
use clama::Event;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};

#[cfg_attr(feature = "clama", derive(Event))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Extent {
    pub(crate) width: NonZeroU32,
    pub(crate) height: NonZeroU32,
}

impl Extent {
    pub const MAX_DIMENSION: u32 = 16_384;
    pub const MAX_AREA: u64 = 67_108_864;
}

impl Default for Extent {
    #[inline]
    fn default() -> Self {
        Self::hd()
    }
}

impl Extent {
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
            width: NonZeroU32::new(width).unwrap(),
            height: NonZeroU32::new(height).unwrap(),
        }
    }

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

    #[inline]
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.width.get() <= Self::MAX_DIMENSION
            && self.height.get() <= Self::MAX_DIMENSION
            && self.area() <= Self::MAX_AREA
    }

    #[inline]
    #[must_use]
    pub const fn is_valid_size(width: u32, height: u32) -> bool {
        width > 0
            && height > 0
            && width <= Self::MAX_DIMENSION
            && height <= Self::MAX_DIMENSION
            && width as u64 * height as u64 <= Self::MAX_AREA
    }

    #[inline]
    #[must_use]
    pub fn with_width(self, width: u32) -> Self {
        Self::clamped(width, self.height())
    }

    #[inline]
    #[must_use]
    pub fn with_height(self, height: u32) -> Self {
        Self::clamped(self.width(), height)
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
    pub const fn aspect_ratio(self) -> f32 {
        self.width.get() as f32 / self.height.get() as f32
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

    #[inline]
    #[must_use]
    pub const fn is_max_dimension(self) -> bool {
        self.width() == Self::MAX_DIMENSION || self.height() == Self::MAX_DIMENSION
    }
}

impl From<[u32; 2]> for Extent {
    #[inline]
    fn from([width, height]: [u32; 2]) -> Self {
        Self::clamped(width, height)
    }
}

impl From<Extent> for [u32; 2] {
    #[inline]
    fn from(size: Extent) -> Self {
        [size.width(), size.height()]
    }
}

impl From<(u32, u32)> for Extent {
    #[inline]
    fn from((width, height): (u32, u32)) -> Self {
        Self::clamped(width, height)
    }
}

impl From<Extent> for (u32, u32) {
    #[inline]
    fn from(size: Extent) -> Self {
        (size.width(), size.height())
    }
}

#[cfg(feature = "serde")]
impl Serialize for Extent {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Extent", 2)?;

        state.serialize_field("width", &self.width())?;
        state.serialize_field("height", &self.height())?;

        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Extent {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawExtent {
            width: u32,
            height: u32,
        }

        let raw = RawExtent::deserialize(deserializer)?;

        Ok(Self::clamped(raw.width, raw.height))
    }
}

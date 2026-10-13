//! Video playback.
//!
//! Wraps the [`ThorVG` C API](https://www.thorvg.org/c-native).

use crate::error::{Error, Result};
use crate::picture::Picture;
use thorvg_sys as sys;

/// Video player that decodes into a [`Picture`].
///
/// Load an MP4 into the picture the video owns (via
/// [`picture_mut`](Self::picture_mut)), add a
/// [`duplicate`](crate::Paint::duplicate) of it to a canvas, then drive
/// playback with [`play`](Self::play), [`pause`](Self::pause) and
/// [`seek`](Self::seek). Decoding runs on the platform media framework
/// in the background; each canvas `update` picks up the latest decoded
/// frame, so render on your own schedule (e.g. per display refresh).
///
/// The duplicate shares the video's decoder, so one added to a canvas
/// follows playback for as long as it stays there.
///
/// *Experimental in `ThorVG`; the API may change.*
///
/// # Platform support
///
/// `ThorVG` ships a video backend only for macOS and iOS
/// (`AVFoundation`), compiled in by the `video` feature. Elsewhere, or
/// with the feature disabled, [`Thorvg::video`](crate::Thorvg::video)
/// returns [`Error::NotSupported`].
///
/// # Example
///
/// ```no_run
/// # fn main() -> Result<(), thorvg::Error> {
/// use thorvg::{Canvas, EngineOption, Paint, Thorvg};
///
/// let engine = Thorvg::init(0)?;
/// let mut canvas = engine.sw_canvas(EngineOption::Default)?;
/// let mut video = engine.video()?;
/// video.picture_mut().load("clip.mp4")?;
/// canvas.add(video.picture().duplicate().unwrap())?;
/// video.play()?;
/// // Per display refresh: canvas.update()?; canvas.draw(true)?; canvas.sync()?;
/// # Ok(())
/// # }
/// ```
///
/// The lifetime `'eng` ties this video to a [`Thorvg`](crate::Thorvg)
/// engine instance. Create videos via
/// [`Thorvg::video()`](crate::Thorvg::video).
///
/// # Thread Safety
///
/// `Video` is [`Send`] but not [`Sync`]: you may move it to another
/// thread, but you must not share references across threads.
pub struct Video<'eng> {
    raw: sys::Tvg_Video,
    // Borrowed picture handle owned by the C-side video (see
    // `Animation::picture`); `owned: false`, so its Drop is a no-op.
    picture: Picture<'eng>,
    _engine: core::marker::PhantomData<&'eng ()>,
}

// SAFETY: `Video` exclusively owns a heap-allocated ThorVG video handle
// (`Tvg_Video`).  The media backend's decoder thread synchronises with
// the handle internally; per-object state is only accessed through
// `&mut self`.  Transferring sole ownership to another thread is safe.
//
// `Video` is intentionally `!Sync`: the raw pointer field prevents the
// auto-`Sync` impl.
unsafe impl Send for Video<'_> {}

impl<'eng> Video<'eng> {
    /// Creates a new Video object.
    pub(crate) fn new() -> Result<Self> {
        // Null when ThorVG was built without a media loader, which is the
        // only way `tvg_video_new` fails short of an allocation abort.
        let raw = unsafe { sys::tvg_video_new() };
        if raw.is_null() {
            return Err(Error::NotSupported);
        }
        let pic_raw = unsafe { sys::tvg_video_get_picture(raw) };
        assert!(!pic_raw.is_null(), "video has no picture");
        Ok(Self {
            raw,
            picture: unsafe { Picture::from_raw(pic_raw, false) },
            _engine: core::marker::PhantomData,
        })
    }

    /// Returns the [`Picture`] this video decodes into.
    ///
    /// The picture is owned by the video and released when the video is
    /// dropped; add a [`duplicate`](crate::Paint::duplicate) of it to a
    /// canvas.
    pub fn picture(&self) -> &Picture<'eng> {
        &self.picture
    }

    /// Returns a mutable borrow of the [`Picture`] this video decodes
    /// into.
    ///
    /// Use this to load the video (e.g. via
    /// [`Picture::load`](crate::Picture::load) or
    /// [`Picture::load_data`](crate::Picture::load_data) with
    /// [`MimeType::Mp4`](crate::MimeType::Mp4)) and to set its size.
    pub fn picture_mut(&mut self) -> &mut Picture<'eng> {
        &mut self.picture
    }

    /// Starts or resumes playback. Playback that reached the end
    /// restarts from the beginning.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InsufficientCondition`] if no video is loaded.
    pub fn play(&mut self) -> Result<()> {
        Error::from_raw(unsafe { sys::tvg_video_play(self.raw) })
    }

    /// Pauses playback at the current position.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InsufficientCondition`] if playback has not
    /// started.
    pub fn pause(&mut self) -> Result<()> {
        Error::from_raw(unsafe { sys::tvg_video_pause(self.raw) })
    }

    /// Stops playback and rewinds to the start.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InsufficientCondition`] if no video is loaded.
    pub fn stop(&mut self) -> Result<()> {
        Error::from_raw(unsafe { sys::tvg_video_stop(self.raw) })
    }

    /// Moves the playback position to `seconds`, in the range
    /// `0.0..=`[`duration`](Self::duration).
    ///
    /// # Errors
    ///
    /// - [`Error::InsufficientCondition`] if no video is loaded.
    /// - [`Error::InvalidArguments`] if `seconds` is out of range.
    pub fn seek(&mut self, seconds: f32) -> Result<()> {
        Error::from_raw(unsafe { sys::tvg_video_seek(self.raw, seconds) })
    }

    /// Enables or disables repeated playback. Off by default.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InsufficientCondition`] if no video is loaded.
    pub fn set_loop(&mut self, on: bool) -> Result<()> {
        Error::from_raw(unsafe { sys::tvg_video_set_loop(self.raw, on) })
    }

    /// Returns `true` if repeated playback is enabled.
    pub fn is_looping(&self) -> bool {
        unsafe { sys::tvg_video_get_loop(self.raw) }
    }

    /// Sets the audio volume in the range `0.0..=1.0`, where `0.0` is
    /// silent and `1.0` (the default) is the original volume.
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidArguments`] if `volume` is out of range.
    /// - [`Error::InsufficientCondition`] if no video is loaded.
    pub fn set_volume(&mut self, volume: f32) -> Result<()> {
        Error::from_raw(unsafe { sys::tvg_video_set_volume(self.raw, volume) })
    }

    /// Returns the audio volume in the range `0.0..=1.0`, or `0.0` if no
    /// video is loaded.
    pub fn volume(&self) -> f32 {
        unsafe { sys::tvg_video_get_volume(self.raw) }
    }

    /// Mutes or unmutes the audio without changing the
    /// [`volume`](Self::volume). Unmuted by default.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InsufficientCondition`] if no video is loaded.
    pub fn set_muted(&mut self, on: bool) -> Result<()> {
        Error::from_raw(unsafe { sys::tvg_video_set_mute(self.raw, on) })
    }

    /// Returns `true` if the audio is muted.
    pub fn is_muted(&self) -> bool {
        unsafe { sys::tvg_video_get_muted(self.raw) }
    }

    /// Returns the current playback position in seconds, or `0.0` if no
    /// video is loaded.
    pub fn time(&self) -> f32 {
        unsafe { sys::tvg_video_get_time(self.raw) }
    }

    /// Returns the video duration in seconds, or `0.0` if no video is
    /// loaded.
    pub fn duration(&self) -> f32 {
        unsafe { sys::tvg_video_get_duration(self.raw) }
    }
}

impl Drop for Video<'_> {
    fn drop(&mut self) {
        unsafe {
            sys::tvg_video_del(self.raw);
        }
    }
}

impl core::fmt::Debug for Video<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Video").finish_non_exhaustive()
    }
}

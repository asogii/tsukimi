// This file was generated by gir (https://github.com/gtk-rs/gir)
// from 
// from ../gir-files-gstreamer
// from ../gir-files-gtk
// DO NOT EDIT

use crate::{Feature,PlayerSeekMethod,PlayerState,Queue,StreamList,ThreadedObject};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "ClapperPlayer")]
    pub struct Player(Object<ffi::ClapperPlayer, ffi::ClapperPlayerClass>) @extends ThreadedObject, gst::Object;

    match fn {
        type_ => || ffi::clapper_player_get_type(),
    }
}

impl Player {
    #[doc(alias = "clapper_player_new")]
    pub fn new() -> Player {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::clapper_player_new())
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Player`] objects.
            ///
            /// This method returns an instance of [`PlayerBuilder`](crate::builders::PlayerBuilder) which can be used to create [`Player`] objects.
            pub fn builder() -> PlayerBuilder {
                PlayerBuilder::new()
            }
        

    #[doc(alias = "clapper_player_add_feature")]
    pub fn add_feature(&self, feature: &impl IsA<Feature>) {
        unsafe {
            ffi::clapper_player_add_feature(self.to_glib_none().0, feature.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "clapper_player_get_audio_enabled")]
    #[doc(alias = "get_audio_enabled")]
    pub fn is_audio_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::clapper_player_get_audio_enabled(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_player_get_audio_filter")]
    #[doc(alias = "get_audio_filter")]
    pub fn audio_filter(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ffi::clapper_player_get_audio_filter(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_player_get_audio_offset")]
    #[doc(alias = "get_audio_offset")]
    pub fn audio_offset(&self) -> f64 {
        unsafe {
            ffi::clapper_player_get_audio_offset(self.to_glib_none().0)
        }
    }

    #[doc(alias = "clapper_player_get_audio_sink")]
    #[doc(alias = "get_audio_sink")]
    pub fn audio_sink(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ffi::clapper_player_get_audio_sink(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_player_get_audio_streams")]
    #[doc(alias = "get_audio_streams")]
    pub fn audio_streams(&self) -> Option<StreamList> {
        unsafe {
            from_glib_none(ffi::clapper_player_get_audio_streams(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_player_get_autoplay")]
    #[doc(alias = "get_autoplay")]
    pub fn is_autoplay(&self) -> bool {
        unsafe {
            from_glib(ffi::clapper_player_get_autoplay(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_player_get_current_audio_decoder")]
    #[doc(alias = "get_current_audio_decoder")]
    pub fn current_audio_decoder(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ffi::clapper_player_get_current_audio_decoder(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_player_get_current_video_decoder")]
    #[doc(alias = "get_current_video_decoder")]
    pub fn current_video_decoder(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ffi::clapper_player_get_current_video_decoder(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_player_get_mute")]
    #[doc(alias = "get_mute")]
    pub fn is_muted(&self) -> bool {
        unsafe {
            from_glib(ffi::clapper_player_get_mute(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_player_get_position")]
    #[doc(alias = "get_position")]
    pub fn position(&self) -> f64 {
        unsafe {
            ffi::clapper_player_get_position(self.to_glib_none().0)
        }
    }

    #[doc(alias = "clapper_player_get_queue")]
    #[doc(alias = "get_queue")]
    pub fn queue(&self) -> Option<Queue> {
        unsafe {
            from_glib_none(ffi::clapper_player_get_queue(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_player_get_speed")]
    #[doc(alias = "get_speed")]
    pub fn speed(&self) -> f64 {
        unsafe {
            ffi::clapper_player_get_speed(self.to_glib_none().0)
        }
    }

    #[doc(alias = "clapper_player_get_state")]
    #[doc(alias = "get_state")]
    pub fn state(&self) -> PlayerState {
        unsafe {
            from_glib(ffi::clapper_player_get_state(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_player_get_subtitle_font_desc")]
    #[doc(alias = "get_subtitle_font_desc")]
    pub fn subtitle_font_desc(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::clapper_player_get_subtitle_font_desc(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_player_get_subtitle_offset")]
    #[doc(alias = "get_subtitle_offset")]
    pub fn subtitle_offset(&self) -> f64 {
        unsafe {
            ffi::clapper_player_get_subtitle_offset(self.to_glib_none().0)
        }
    }

    #[doc(alias = "clapper_player_get_subtitle_streams")]
    #[doc(alias = "get_subtitle_streams")]
    pub fn subtitle_streams(&self) -> Option<StreamList> {
        unsafe {
            from_glib_none(ffi::clapper_player_get_subtitle_streams(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_player_get_subtitles_enabled")]
    #[doc(alias = "get_subtitles_enabled")]
    pub fn is_subtitles_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::clapper_player_get_subtitles_enabled(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_player_get_video_enabled")]
    #[doc(alias = "get_video_enabled")]
    pub fn is_video_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::clapper_player_get_video_enabled(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_player_get_video_filter")]
    #[doc(alias = "get_video_filter")]
    pub fn video_filter(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ffi::clapper_player_get_video_filter(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_player_get_video_sink")]
    #[doc(alias = "get_video_sink")]
    pub fn video_sink(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ffi::clapper_player_get_video_sink(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_player_get_video_streams")]
    #[doc(alias = "get_video_streams")]
    pub fn video_streams(&self) -> Option<StreamList> {
        unsafe {
            from_glib_none(ffi::clapper_player_get_video_streams(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_player_get_volume")]
    #[doc(alias = "get_volume")]
    pub fn volume(&self) -> f64 {
        unsafe {
            ffi::clapper_player_get_volume(self.to_glib_none().0)
        }
    }

    #[doc(alias = "clapper_player_pause")]
    pub fn pause(&self) {
        unsafe {
            ffi::clapper_player_pause(self.to_glib_none().0);
        }
    }

    #[doc(alias = "clapper_player_play")]
    pub fn play(&self) {
        unsafe {
            ffi::clapper_player_play(self.to_glib_none().0);
        }
    }

    #[doc(alias = "clapper_player_seek")]
    pub fn seek(&self, position: f64) {
        unsafe {
            ffi::clapper_player_seek(self.to_glib_none().0, position);
        }
    }

    #[doc(alias = "clapper_player_seek_custom")]
    pub fn seek_custom(&self, position: f64, method: PlayerSeekMethod) {
        unsafe {
            ffi::clapper_player_seek_custom(self.to_glib_none().0, position, method.into_glib());
        }
    }

    #[doc(alias = "clapper_player_set_audio_enabled")]
    pub fn set_audio_enabled(&self, enabled: bool) {
        unsafe {
            ffi::clapper_player_set_audio_enabled(self.to_glib_none().0, enabled.into_glib());
        }
    }

    #[doc(alias = "clapper_player_set_audio_filter")]
    pub fn set_audio_filter(&self, element: Option<&impl IsA<gst::Element>>) {
        unsafe {
            ffi::clapper_player_set_audio_filter(self.to_glib_none().0, element.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    #[doc(alias = "clapper_player_set_audio_offset")]
    pub fn set_audio_offset(&self, offset: f64) {
        unsafe {
            ffi::clapper_player_set_audio_offset(self.to_glib_none().0, offset);
        }
    }

    #[doc(alias = "clapper_player_set_audio_sink")]
    pub fn set_audio_sink(&self, element: Option<&impl IsA<gst::Element>>) {
        unsafe {
            ffi::clapper_player_set_audio_sink(self.to_glib_none().0, element.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    #[doc(alias = "clapper_player_set_autoplay")]
    pub fn set_autoplay(&self, enabled: bool) {
        unsafe {
            ffi::clapper_player_set_autoplay(self.to_glib_none().0, enabled.into_glib());
        }
    }

    #[doc(alias = "clapper_player_set_mute")]
    pub fn set_mute(&self, mute: bool) {
        unsafe {
            ffi::clapper_player_set_mute(self.to_glib_none().0, mute.into_glib());
        }
    }

    #[doc(alias = "clapper_player_set_speed")]
    pub fn set_speed(&self, speed: f64) {
        unsafe {
            ffi::clapper_player_set_speed(self.to_glib_none().0, speed);
        }
    }

    #[doc(alias = "clapper_player_set_subtitle_font_desc")]
    pub fn set_subtitle_font_desc(&self, font_desc: &str) {
        unsafe {
            ffi::clapper_player_set_subtitle_font_desc(self.to_glib_none().0, font_desc.to_glib_none().0);
        }
    }

    #[doc(alias = "clapper_player_set_subtitle_offset")]
    pub fn set_subtitle_offset(&self, offset: f64) {
        unsafe {
            ffi::clapper_player_set_subtitle_offset(self.to_glib_none().0, offset);
        }
    }

    #[doc(alias = "clapper_player_set_subtitles_enabled")]
    pub fn set_subtitles_enabled(&self, enabled: bool) {
        unsafe {
            ffi::clapper_player_set_subtitles_enabled(self.to_glib_none().0, enabled.into_glib());
        }
    }

    #[doc(alias = "clapper_player_set_video_enabled")]
    pub fn set_video_enabled(&self, enabled: bool) {
        unsafe {
            ffi::clapper_player_set_video_enabled(self.to_glib_none().0, enabled.into_glib());
        }
    }

    #[doc(alias = "clapper_player_set_video_filter")]
    pub fn set_video_filter(&self, element: Option<&impl IsA<gst::Element>>) {
        unsafe {
            ffi::clapper_player_set_video_filter(self.to_glib_none().0, element.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    #[doc(alias = "clapper_player_set_video_sink")]
    pub fn set_video_sink(&self, element: Option<&impl IsA<gst::Element>>) {
        unsafe {
            ffi::clapper_player_set_video_sink(self.to_glib_none().0, element.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    #[doc(alias = "clapper_player_set_volume")]
    pub fn set_volume(&self, volume: f64) {
        unsafe {
            ffi::clapper_player_set_volume(self.to_glib_none().0, volume);
        }
    }

    #[doc(alias = "clapper_player_stop")]
    pub fn stop(&self) {
        unsafe {
            ffi::clapper_player_stop(self.to_glib_none().0);
        }
    }

    //#[doc(alias = "error")]
    //pub fn connect_error<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored error: GLib.Error
    //}

    #[doc(alias = "missing-plugin")]
    pub fn connect_missing_plugin<F: Fn(&Self, &str, Option<&str>) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn missing_plugin_trampoline<F: Fn(&Player, &str, Option<&str>) + 'static>(this: *mut ffi::ClapperPlayer, name: *mut libc::c_char, installer_detail: *mut libc::c_char, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &glib::GString::from_glib_borrow(name), Option::<glib::GString>::from_glib_borrow(installer_detail).as_ref().as_ref().map(|s| s.as_str()))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"missing-plugin\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(missing_plugin_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "seek-done")]
    pub fn connect_seek_done<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn seek_done_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"seek-done\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(seek_done_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    //#[doc(alias = "warning")]
    //pub fn connect_warning<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored error: GLib.Error
    //}

    #[doc(alias = "audio-enabled")]
    pub fn connect_audio_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_audio_enabled_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::audio-enabled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_audio_enabled_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "audio-filter")]
    pub fn connect_audio_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_audio_filter_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::audio-filter\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_audio_filter_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "audio-offset")]
    pub fn connect_audio_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_audio_offset_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::audio-offset\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_audio_offset_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "audio-sink")]
    pub fn connect_audio_sink_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_audio_sink_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::audio-sink\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_audio_sink_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "audio-streams")]
    pub fn connect_audio_streams_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_audio_streams_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::audio-streams\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_audio_streams_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "autoplay")]
    pub fn connect_autoplay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_autoplay_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::autoplay\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_autoplay_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "current-audio-decoder")]
    pub fn connect_current_audio_decoder_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_current_audio_decoder_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::current-audio-decoder\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_current_audio_decoder_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "current-video-decoder")]
    pub fn connect_current_video_decoder_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_current_video_decoder_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::current-video-decoder\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_current_video_decoder_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "mute")]
    pub fn connect_mute_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mute_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::mute\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_mute_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "position")]
    pub fn connect_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::position\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_position_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "queue")]
    pub fn connect_queue_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_queue_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::queue\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_queue_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "speed")]
    pub fn connect_speed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_speed_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::speed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_speed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "state")]
    pub fn connect_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::state\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_state_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "subtitle-font-desc")]
    pub fn connect_subtitle_font_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_font_desc_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::subtitle-font-desc\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_subtitle_font_desc_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "subtitle-offset")]
    pub fn connect_subtitle_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_offset_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::subtitle-offset\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_subtitle_offset_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "subtitle-streams")]
    pub fn connect_subtitle_streams_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_streams_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::subtitle-streams\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_subtitle_streams_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "subtitles-enabled")]
    pub fn connect_subtitles_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitles_enabled_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::subtitles-enabled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_subtitles_enabled_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "video-enabled")]
    pub fn connect_video_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_video_enabled_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::video-enabled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_video_enabled_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "video-filter")]
    pub fn connect_video_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_video_filter_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::video-filter\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_video_filter_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "video-sink")]
    pub fn connect_video_sink_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_video_sink_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::video-sink\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_video_sink_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "video-streams")]
    pub fn connect_video_streams_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_video_streams_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::video-streams\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_video_streams_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "volume")]
    pub fn connect_volume_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_volume_trampoline<F: Fn(&Player) + 'static>(this: *mut ffi::ClapperPlayer, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::volume\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_volume_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl Default for Player {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Player`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PlayerBuilder {
            builder: glib::object::ObjectBuilder<'static, Player>,
        }

        impl PlayerBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn audio_enabled(self, audio_enabled: bool) -> Self {
                            Self { builder: self.builder.property("audio-enabled", audio_enabled), }
                        }

                            pub fn audio_filter(self, audio_filter: &impl IsA<gst::Element>) -> Self {
                            Self { builder: self.builder.property("audio-filter", audio_filter.clone().upcast()), }
                        }

                            pub fn audio_offset(self, audio_offset: f64) -> Self {
                            Self { builder: self.builder.property("audio-offset", audio_offset), }
                        }

                            pub fn audio_sink(self, audio_sink: &impl IsA<gst::Element>) -> Self {
                            Self { builder: self.builder.property("audio-sink", audio_sink.clone().upcast()), }
                        }

                            pub fn autoplay(self, autoplay: bool) -> Self {
                            Self { builder: self.builder.property("autoplay", autoplay), }
                        }

                            pub fn mute(self, mute: bool) -> Self {
                            Self { builder: self.builder.property("mute", mute), }
                        }

                            pub fn speed(self, speed: f64) -> Self {
                            Self { builder: self.builder.property("speed", speed), }
                        }

                            pub fn subtitle_font_desc(self, subtitle_font_desc: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("subtitle-font-desc", subtitle_font_desc.into()), }
                        }

                            pub fn subtitle_offset(self, subtitle_offset: f64) -> Self {
                            Self { builder: self.builder.property("subtitle-offset", subtitle_offset), }
                        }

                            pub fn subtitles_enabled(self, subtitles_enabled: bool) -> Self {
                            Self { builder: self.builder.property("subtitles-enabled", subtitles_enabled), }
                        }

                            pub fn video_enabled(self, video_enabled: bool) -> Self {
                            Self { builder: self.builder.property("video-enabled", video_enabled), }
                        }

                            pub fn video_filter(self, video_filter: &impl IsA<gst::Element>) -> Self {
                            Self { builder: self.builder.property("video-filter", video_filter.clone().upcast()), }
                        }

                            pub fn video_sink(self, video_sink: &impl IsA<gst::Element>) -> Self {
                            Self { builder: self.builder.property("video-sink", video_sink.clone().upcast()), }
                        }

                            pub fn volume(self, volume: f64) -> Self {
                            Self { builder: self.builder.property("volume", volume), }
                        }

                            pub fn name(self, name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("name", name.into()), }
                        }

                            pub fn parent(self, parent: &impl IsA<gst::Object>) -> Self {
                            Self { builder: self.builder.property("parent", parent.clone().upcast()), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`Player`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Player {
    self.builder.build() }
}
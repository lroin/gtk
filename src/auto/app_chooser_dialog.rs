// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use AppChooser;
use Bin;
use Buildable;
use Container;
use Dialog;
use DialogFlags;
use Widget;
use Window;
use ffi;
use gio;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct AppChooserDialog(Object<ffi::GtkAppChooserDialog, ffi::GtkAppChooserDialogClass>): Dialog, Window, Bin, Container, Widget, Buildable, AppChooser;

    match fn {
        get_type => || ffi::gtk_app_chooser_dialog_get_type(),
    }
}

impl AppChooserDialog {
    pub fn new<'a, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>, R: IsA<gio::File>>(parent: Q, flags: DialogFlags, file: &R) -> AppChooserDialog {
        assert_initialized_main_thread!();
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_app_chooser_dialog_new(parent.0, flags.to_glib(), file.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_for_content_type<'a, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>>(parent: Q, flags: DialogFlags, content_type: &str) -> AppChooserDialog {
        assert_initialized_main_thread!();
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_app_chooser_dialog_new_for_content_type(parent.0, flags.to_glib(), content_type.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait AppChooserDialogExt {
    fn get_heading(&self) -> Option<String>;

    fn get_widget(&self) -> Widget;

    fn set_heading(&self, heading: &str);

    fn get_property_gfile(&self) -> Option<gio::File>;

    fn connect_property_heading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AppChooserDialog> + IsA<glib::object::Object>> AppChooserDialogExt for O {
    fn get_heading(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_app_chooser_dialog_get_heading(self.to_glib_none().0))
        }
    }

    fn get_widget(&self) -> Widget {
        unsafe {
            from_glib_none(ffi::gtk_app_chooser_dialog_get_widget(self.to_glib_none().0))
        }
    }

    fn set_heading(&self, heading: &str) {
        unsafe {
            ffi::gtk_app_chooser_dialog_set_heading(self.to_glib_none().0, heading.to_glib_none().0);
        }
    }

    fn get_property_gfile(&self) -> Option<gio::File> {
        unsafe {
            let mut value = Value::from_type(<gio::File as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "gfile".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_property_heading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::heading",
                transmute(notify_heading_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_heading_trampoline<P>(this: *mut ffi::GtkAppChooserDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AppChooserDialog> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AppChooserDialog::from_glib_borrow(this).downcast_unchecked())
}

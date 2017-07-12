// This file was generated by gir (4b09025) from gir-files (71d73f0)
// DO NOT EDIT

use AppChooser;
use Bin;
use Container;
use Dialog;
use DialogFlags;
use Widget;
use Window;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    pub struct AppChooserDialog(Object<ffi::GtkAppChooserDialog>): Dialog, Window, Bin, Container, Widget, AppChooser;

    match fn {
        get_type => || ffi::gtk_app_chooser_dialog_get_type(),
    }
}

impl AppChooserDialog {
    //pub fn new<'a, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>, R: IsA</*Ignored*/gio::File>>(parent: Q, flags: DialogFlags, file: &R) -> AppChooserDialog {
    //    unsafe { TODO: call ffi::gtk_app_chooser_dialog_new() }
    //}

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

    //fn get_property_gfile(&self) -> /*Ignored*/Option<gio::File>;
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

    //fn get_property_gfile(&self) -> /*Ignored*/Option<gio::File> {
    //    let mut value = Value::from(None::<&/*Ignored*/gio::File>);
    //    unsafe {
    //        gobject_ffi::g_object_get_property(self.to_glib_none().0, "gfile".to_glib_none().0, value.to_glib_none_mut().0);
    //    }
    //    value.get()
    //}
}

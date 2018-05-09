// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
use File;
use IOErrorEnum;
use IOStream;
use Icon;
use InputStream;
use Resource;
use ResourceLookupFlags;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std;
#[cfg(feature = "futures")]
use std::boxed::Box as Box_;
use std::mem;
use std::ptr;


//pub fn bus_get<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result</*Ignored*/DBusConnection, Error>) + Send + 'static>(bus_type: /*Ignored*/BusType, cancellable: P, callback: Q) {
//    unsafe { TODO: call ffi::g_bus_get() }
//}

//#[cfg(feature = "futures")]
//pub fn bus_get_future(bus_type: /*Ignored*/BusType) -> Box_<futures_core::Future<Item = /*Ignored*/DBusConnection, Error = Error>> {
    //use GioFuture;
    //use send_cell::SendCell;

    //GioFuture::new(&(), move |_obj, send| {
    //    let cancellable = Cancellable::new();
    //    let send = SendCell::new(send);
    //    bus_get(
    //         bus_type,
    //         Some(&cancellable),
    //         move |res| {
    //             let _ = send.into_inner().send(res);
    //         },
    //    );

    //    cancellable
    //})
//}

//pub fn bus_get_sync<'a, P: Into<Option<&'a Cancellable>>>(bus_type: /*Ignored*/BusType, cancellable: P) -> Result</*Ignored*/DBusConnection, Error> {
//    unsafe { TODO: call ffi::g_bus_get_sync() }
//}

//pub fn bus_own_name<'a, 'b, 'c, 'd, P: Into<Option<&'a /*Unimplemented*/BusAcquiredCallback>>, Q: Into<Option<&'b /*Unimplemented*/BusNameAcquiredCallback>>, R: Into<Option<&'c /*Unimplemented*/BusNameLostCallback>>, S: Into<Option<&'d /*Ignored*/glib::DestroyNotify>>>(bus_type: /*Ignored*/BusType, name: &str, flags: /*Ignored*/BusNameOwnerFlags, bus_acquired_handler: P, name_acquired_handler: Q, name_lost_handler: R, user_data_free_func: S) -> u32 {
//    unsafe { TODO: call ffi::g_bus_own_name() }
//}

//pub fn bus_own_name_on_connection<'a, 'b, 'c, P: Into<Option<&'a /*Unimplemented*/BusNameAcquiredCallback>>, Q: Into<Option<&'b /*Unimplemented*/BusNameLostCallback>>, R: Into<Option<&'c /*Ignored*/glib::DestroyNotify>>>(connection: /*Ignored*/&DBusConnection, name: &str, flags: /*Ignored*/BusNameOwnerFlags, name_acquired_handler: P, name_lost_handler: Q, user_data_free_func: R) -> u32 {
//    unsafe { TODO: call ffi::g_bus_own_name_on_connection() }
//}

//pub fn bus_own_name_on_connection_with_closures<'a, 'b, P: Into<Option<&'a /*Ignored*/glib::Closure>>, Q: Into<Option<&'b /*Ignored*/glib::Closure>>>(connection: /*Ignored*/&DBusConnection, name: &str, flags: /*Ignored*/BusNameOwnerFlags, name_acquired_closure: P, name_lost_closure: Q) -> u32 {
//    unsafe { TODO: call ffi::g_bus_own_name_on_connection_with_closures() }
//}

//pub fn bus_own_name_with_closures<'a, 'b, 'c, P: Into<Option<&'a /*Ignored*/glib::Closure>>, Q: Into<Option<&'b /*Ignored*/glib::Closure>>, R: Into<Option<&'c /*Ignored*/glib::Closure>>>(bus_type: /*Ignored*/BusType, name: &str, flags: /*Ignored*/BusNameOwnerFlags, bus_acquired_closure: P, name_acquired_closure: Q, name_lost_closure: R) -> u32 {
//    unsafe { TODO: call ffi::g_bus_own_name_with_closures() }
//}

pub fn bus_unown_name(owner_id: u32) {
    unsafe {
        ffi::g_bus_unown_name(owner_id);
    }
}

pub fn bus_unwatch_name(watcher_id: u32) {
    unsafe {
        ffi::g_bus_unwatch_name(watcher_id);
    }
}

//pub fn bus_watch_name<'a, 'b, 'c, P: Into<Option<&'a /*Unimplemented*/BusNameAppearedCallback>>, Q: Into<Option<&'b /*Unimplemented*/BusNameVanishedCallback>>, R: Into<Option<&'c /*Ignored*/glib::DestroyNotify>>>(bus_type: /*Ignored*/BusType, name: &str, flags: /*Ignored*/BusNameWatcherFlags, name_appeared_handler: P, name_vanished_handler: Q, user_data_free_func: R) -> u32 {
//    unsafe { TODO: call ffi::g_bus_watch_name() }
//}

//pub fn bus_watch_name_on_connection<'a, 'b, 'c, P: Into<Option<&'a /*Unimplemented*/BusNameAppearedCallback>>, Q: Into<Option<&'b /*Unimplemented*/BusNameVanishedCallback>>, R: Into<Option<&'c /*Ignored*/glib::DestroyNotify>>>(connection: /*Ignored*/&DBusConnection, name: &str, flags: /*Ignored*/BusNameWatcherFlags, name_appeared_handler: P, name_vanished_handler: Q, user_data_free_func: R) -> u32 {
//    unsafe { TODO: call ffi::g_bus_watch_name_on_connection() }
//}

//pub fn bus_watch_name_on_connection_with_closures<'a, 'b, P: Into<Option<&'a /*Ignored*/glib::Closure>>, Q: Into<Option<&'b /*Ignored*/glib::Closure>>>(connection: /*Ignored*/&DBusConnection, name: &str, flags: /*Ignored*/BusNameWatcherFlags, name_appeared_closure: P, name_vanished_closure: Q) -> u32 {
//    unsafe { TODO: call ffi::g_bus_watch_name_on_connection_with_closures() }
//}

//pub fn bus_watch_name_with_closures<'a, 'b, P: Into<Option<&'a /*Ignored*/glib::Closure>>, Q: Into<Option<&'b /*Ignored*/glib::Closure>>>(bus_type: /*Ignored*/BusType, name: &str, flags: /*Ignored*/BusNameWatcherFlags, name_appeared_closure: P, name_vanished_closure: Q) -> u32 {
//    unsafe { TODO: call ffi::g_bus_watch_name_with_closures() }
//}

pub fn content_type_can_be_executable(type_: &str) -> bool {
    unsafe {
        from_glib(ffi::g_content_type_can_be_executable(type_.to_glib_none().0))
    }
}

pub fn content_type_equals(type1: &str, type2: &str) -> bool {
    unsafe {
        from_glib(ffi::g_content_type_equals(type1.to_glib_none().0, type2.to_glib_none().0))
    }
}

pub fn content_type_from_mime_type(mime_type: &str) -> Option<String> {
    unsafe {
        from_glib_full(ffi::g_content_type_from_mime_type(mime_type.to_glib_none().0))
    }
}

pub fn content_type_get_description(type_: &str) -> Option<String> {
    unsafe {
        from_glib_full(ffi::g_content_type_get_description(type_.to_glib_none().0))
    }
}

#[cfg(any(feature = "v2_34", feature = "dox"))]
pub fn content_type_get_generic_icon_name(type_: &str) -> Option<String> {
    unsafe {
        from_glib_full(ffi::g_content_type_get_generic_icon_name(type_.to_glib_none().0))
    }
}

pub fn content_type_get_icon(type_: &str) -> Option<Icon> {
    unsafe {
        from_glib_full(ffi::g_content_type_get_icon(type_.to_glib_none().0))
    }
}

pub fn content_type_get_mime_type(type_: &str) -> Option<String> {
    unsafe {
        from_glib_full(ffi::g_content_type_get_mime_type(type_.to_glib_none().0))
    }
}

#[cfg(any(feature = "v2_34", feature = "dox"))]
pub fn content_type_get_symbolic_icon(type_: &str) -> Option<Icon> {
    unsafe {
        from_glib_full(ffi::g_content_type_get_symbolic_icon(type_.to_glib_none().0))
    }
}

pub fn content_type_guess<'a, P: Into<Option<&'a str>>>(filename: P, data: &[u8]) -> (String, bool) {
    let filename = filename.into();
    let filename = filename.to_glib_none();
    let data_size = data.len() as usize;
    unsafe {
        let mut result_uncertain = mem::uninitialized();
        let ret = from_glib_full(ffi::g_content_type_guess(filename.0, data.to_glib_none().0, data_size, &mut result_uncertain));
        (ret, from_glib(result_uncertain))
    }
}

pub fn content_type_guess_for_tree<P: IsA<File>>(root: &P) -> Vec<String> {
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::g_content_type_guess_for_tree(root.to_glib_none().0))
    }
}

pub fn content_type_is_a(type_: &str, supertype: &str) -> bool {
    unsafe {
        from_glib(ffi::g_content_type_is_a(type_.to_glib_none().0, supertype.to_glib_none().0))
    }
}

#[cfg(any(feature = "v2_52", feature = "dox"))]
pub fn content_type_is_mime_type(type_: &str, mime_type: &str) -> bool {
    unsafe {
        from_glib(ffi::g_content_type_is_mime_type(type_.to_glib_none().0, mime_type.to_glib_none().0))
    }
}

pub fn content_type_is_unknown(type_: &str) -> bool {
    unsafe {
        from_glib(ffi::g_content_type_is_unknown(type_.to_glib_none().0))
    }
}

pub fn content_types_get_registered() -> Vec<String> {
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::g_content_types_get_registered())
    }
}

#[cfg(any(feature = "v2_36", feature = "dox"))]
pub fn dbus_address_escape_value(string: &str) -> Option<String> {
    unsafe {
        from_glib_full(ffi::g_dbus_address_escape_value(string.to_glib_none().0))
    }
}

//pub fn dbus_address_get_for_bus_sync<'a, P: Into<Option<&'a Cancellable>>>(bus_type: /*Ignored*/BusType, cancellable: P) -> Result<String, Error> {
//    unsafe { TODO: call ffi::g_dbus_address_get_for_bus_sync() }
//}

pub fn dbus_address_get_stream<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(IOStream, String), Error>) + Send + 'static>(address: &str, cancellable: P, callback: Q) {
    let cancellable = cancellable.into();
    let cancellable = cancellable.to_glib_none();
    let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
    unsafe extern "C" fn dbus_address_get_stream_trampoline<Q: FnOnce(Result<(IOStream, String), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
    {
        callback_guard!();
        let mut error = ptr::null_mut();
        let mut out_guid = ptr::null_mut();
        let ret = ffi::g_dbus_address_get_stream_finish(res, &mut out_guid, &mut error);
        let result = if error.is_null() { Ok((from_glib_full(ret), from_glib_full(out_guid))) } else { Err(from_glib_full(error)) };
        let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
        callback(result);
    }
    let callback = dbus_address_get_stream_trampoline::<Q>;
    unsafe {
        ffi::g_dbus_address_get_stream(address.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
    }
}

#[cfg(feature = "futures")]
pub fn dbus_address_get_stream_future(address: &str) -> Box_<futures_core::Future<Item = (IOStream, String), Error = Error>> {
    use GioFuture;
    use send_cell::SendCell;

    let address = String::from(address);
    GioFuture::new(&(), move |_obj, send| {
        let cancellable = Cancellable::new();
        let send = SendCell::new(send);
        dbus_address_get_stream(
             &address,
             Some(&cancellable),
             move |res| {
                 let _ = send.into_inner().send(res);
             },
        );

        cancellable
    })
}

pub fn dbus_address_get_stream_sync<'a, P: Into<Option<&'a Cancellable>>>(address: &str, cancellable: P) -> Result<(IOStream, String), Error> {
    let cancellable = cancellable.into();
    let cancellable = cancellable.to_glib_none();
    unsafe {
        let mut out_guid = ptr::null_mut();
        let mut error = ptr::null_mut();
        let ret = ffi::g_dbus_address_get_stream_sync(address.to_glib_none().0, &mut out_guid, cancellable.0, &mut error);
        if error.is_null() { Ok((from_glib_full(ret), from_glib_full(out_guid))) } else { Err(from_glib_full(error)) }
    }
}

pub fn dbus_generate_guid() -> Option<String> {
    unsafe {
        from_glib_full(ffi::g_dbus_generate_guid())
    }
}

//pub fn dbus_gvalue_to_gvariant(gvalue: /*Ignored*/&glib::Value, type_: &glib::VariantTy) -> Option<glib::Variant> {
//    unsafe { TODO: call ffi::g_dbus_gvalue_to_gvariant() }
//}

//pub fn dbus_gvariant_to_gvalue(value: &glib::Variant, out_gvalue: /*Ignored*/glib::Value) {
//    unsafe { TODO: call ffi::g_dbus_gvariant_to_gvalue() }
//}

pub fn dbus_is_address(string: &str) -> bool {
    unsafe {
        from_glib(ffi::g_dbus_is_address(string.to_glib_none().0))
    }
}

pub fn dbus_is_guid(string: &str) -> bool {
    unsafe {
        from_glib(ffi::g_dbus_is_guid(string.to_glib_none().0))
    }
}

pub fn dbus_is_interface_name(string: &str) -> bool {
    unsafe {
        from_glib(ffi::g_dbus_is_interface_name(string.to_glib_none().0))
    }
}

pub fn dbus_is_member_name(string: &str) -> bool {
    unsafe {
        from_glib(ffi::g_dbus_is_member_name(string.to_glib_none().0))
    }
}

pub fn dbus_is_name(string: &str) -> bool {
    unsafe {
        from_glib(ffi::g_dbus_is_name(string.to_glib_none().0))
    }
}

pub fn dbus_is_supported_address(string: &str) -> Result<(), Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::g_dbus_is_supported_address(string.to_glib_none().0, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}

pub fn dbus_is_unique_name(string: &str) -> bool {
    unsafe {
        from_glib(ffi::g_dbus_is_unique_name(string.to_glib_none().0))
    }
}

pub fn io_error_from_errno(err_no: i32) -> IOErrorEnum {
    unsafe {
        from_glib(ffi::g_io_error_from_errno(err_no))
    }
}

//pub fn io_error_quark() -> /*Ignored*/glib::Quark {
//    unsafe { TODO: call ffi::g_io_error_quark() }
//}

//pub fn io_modules_load_all_in_directory<P: AsRef<std::path::Path>>(dirname: P) -> /*Ignored*/Vec<IOModule> {
//    unsafe { TODO: call ffi::g_io_modules_load_all_in_directory() }
//}

//pub fn io_modules_load_all_in_directory_with_scope<P: AsRef<std::path::Path>>(dirname: P, scope: /*Ignored*/&mut IOModuleScope) -> /*Ignored*/Vec<IOModule> {
//    unsafe { TODO: call ffi::g_io_modules_load_all_in_directory_with_scope() }
//}

pub fn io_modules_scan_all_in_directory<P: AsRef<std::path::Path>>(dirname: P) {
    unsafe {
        ffi::g_io_modules_scan_all_in_directory(dirname.as_ref().to_glib_none().0);
    }
}

//pub fn io_modules_scan_all_in_directory_with_scope<P: AsRef<std::path::Path>>(dirname: P, scope: /*Ignored*/&mut IOModuleScope) {
//    unsafe { TODO: call ffi::g_io_modules_scan_all_in_directory_with_scope() }
//}

pub fn io_scheduler_cancel_all_jobs() {
    unsafe {
        ffi::g_io_scheduler_cancel_all_jobs();
    }
}

//pub fn io_scheduler_push_job<'a, 'b, P: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>, Q: Into<Option<&'b Cancellable>>>(job_func: /*Unknown conversion*//*Unimplemented*/IOSchedulerJobFunc, notify: P, io_priority: i32, cancellable: Q) {
//    unsafe { TODO: call ffi::g_io_scheduler_push_job() }
//}

//pub fn keyfile_settings_backend_new<'a, P: Into<Option<&'a str>>>(filename: &str, root_path: &str, root_group: P) -> /*Ignored*/Option<SettingsBackend> {
//    unsafe { TODO: call ffi::g_keyfile_settings_backend_new() }
//}

//pub fn memory_settings_backend_new() -> /*Ignored*/Option<SettingsBackend> {
//    unsafe { TODO: call ffi::g_memory_settings_backend_new() }
//}

#[cfg(any(feature = "v2_36", feature = "dox"))]
pub fn networking_init() {
    unsafe {
        ffi::g_networking_init();
    }
}

//pub fn null_settings_backend_new() -> /*Ignored*/Option<SettingsBackend> {
//    unsafe { TODO: call ffi::g_null_settings_backend_new() }
//}

pub fn resources_enumerate_children(path: &str, lookup_flags: ResourceLookupFlags) -> Result<Vec<String>, Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::g_resources_enumerate_children(path.to_glib_none().0, lookup_flags.to_glib(), &mut error);
        if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) }
    }
}

pub fn resources_get_info(path: &str, lookup_flags: ResourceLookupFlags) -> Result<(usize, u32), Error> {
    unsafe {
        let mut size = mem::uninitialized();
        let mut flags = mem::uninitialized();
        let mut error = ptr::null_mut();
        let _ = ffi::g_resources_get_info(path.to_glib_none().0, lookup_flags.to_glib(), &mut size, &mut flags, &mut error);
        if error.is_null() { Ok((size, flags)) } else { Err(from_glib_full(error)) }
    }
}

pub fn resources_lookup_data(path: &str, lookup_flags: ResourceLookupFlags) -> Result<glib::Bytes, Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::g_resources_lookup_data(path.to_glib_none().0, lookup_flags.to_glib(), &mut error);
        if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
    }
}

pub fn resources_open_stream(path: &str, lookup_flags: ResourceLookupFlags) -> Result<InputStream, Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::g_resources_open_stream(path.to_glib_none().0, lookup_flags.to_glib(), &mut error);
        if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
    }
}

pub fn resources_register(resource: &Resource) {
    unsafe {
        ffi::g_resources_register(resource.to_glib_none().0);
    }
}

pub fn resources_unregister(resource: &Resource) {
    unsafe {
        ffi::g_resources_unregister(resource.to_glib_none().0);
    }
}

//#[cfg_attr(feature = "v2_46", deprecated)]
//pub fn simple_async_report_error_in_idle<'a, P: IsA<glib::Object> + 'a, Q: Into<Option<&'a P>>, R: /*Unimplemented*/AsyncReadyCallback>(object: Q, callback: R, domain: /*Ignored*/glib::Quark, code: i32, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi::g_simple_async_report_error_in_idle() }
//}

//#[cfg_attr(feature = "v2_46", deprecated)]
//pub fn simple_async_report_gerror_in_idle<'a, P: IsA<glib::Object> + 'a, Q: Into<Option<&'a P>>, R: /*Unimplemented*/AsyncReadyCallback>(object: Q, callback: R, error: &Error) {
//    unsafe { TODO: call ffi::g_simple_async_report_gerror_in_idle() }
//}

//#[cfg_attr(feature = "v2_46", deprecated)]
//pub fn simple_async_report_take_gerror_in_idle<'a, P: IsA<glib::Object> + 'a, Q: Into<Option<&'a P>>, R: /*Unimplemented*/AsyncReadyCallback>(object: Q, callback: R, error: &mut Error) {
//    unsafe { TODO: call ffi::g_simple_async_report_take_gerror_in_idle() }
//}

#[cfg(any(unix, feature = "dox"))]
pub fn unix_is_mount_path_system_internal<P: AsRef<std::path::Path>>(mount_path: P) -> bool {
    unsafe {
        from_glib(ffi::g_unix_is_mount_path_system_internal(mount_path.as_ref().to_glib_none().0))
    }
}

#[cfg(any(unix, feature = "dox"))]
#[cfg(any(feature = "v2_56", feature = "dox"))]
pub fn unix_is_system_device_path<P: AsRef<std::path::Path>>(device_path: P) -> bool {
    unsafe {
        from_glib(ffi::g_unix_is_system_device_path(device_path.as_ref().to_glib_none().0))
    }
}

#[cfg(any(unix, feature = "dox"))]
#[cfg(any(feature = "v2_56", feature = "dox"))]
pub fn unix_is_system_fs_type(fs_type: &str) -> bool {
    unsafe {
        from_glib(ffi::g_unix_is_system_fs_type(fs_type.to_glib_none().0))
    }
}

//#[cfg(any(unix, feature = "dox"))]
//pub fn unix_mount_at<P: AsRef<std::path::Path>>(mount_path: P) -> (/*Ignored*/UnixMountEntry, u64) {
//    unsafe { TODO: call ffi::g_unix_mount_at() }
//}

//#[cfg(any(unix, feature = "dox"))]
//pub fn unix_mount_compare(mount1: /*Ignored*/&mut UnixMountEntry, mount2: /*Ignored*/&mut UnixMountEntry) -> i32 {
//    unsafe { TODO: call ffi::g_unix_mount_compare() }
//}

//#[cfg(any(unix, feature = "dox"))]
//#[cfg(any(feature = "v2_54", feature = "dox"))]
//pub fn unix_mount_copy(mount_entry: /*Ignored*/&mut UnixMountEntry) -> /*Ignored*/Option<UnixMountEntry> {
//    unsafe { TODO: call ffi::g_unix_mount_copy() }
//}

//#[cfg(any(unix, feature = "dox"))]
//#[cfg(any(feature = "v2_52", feature = "dox"))]
//pub fn unix_mount_for<P: AsRef<std::path::Path>>(file_path: P) -> (/*Ignored*/UnixMountEntry, u64) {
//    unsafe { TODO: call ffi::g_unix_mount_for() }
//}

//#[cfg(any(unix, feature = "dox"))]
//pub fn unix_mount_free(mount_entry: /*Ignored*/&mut UnixMountEntry) {
//    unsafe { TODO: call ffi::g_unix_mount_free() }
//}

//#[cfg(any(unix, feature = "dox"))]
//pub fn unix_mount_get_device_path(mount_entry: /*Ignored*/&mut UnixMountEntry) -> Option<std::path::PathBuf> {
//    unsafe { TODO: call ffi::g_unix_mount_get_device_path() }
//}

//#[cfg(any(unix, feature = "dox"))]
//pub fn unix_mount_get_fs_type(mount_entry: /*Ignored*/&mut UnixMountEntry) -> Option<String> {
//    unsafe { TODO: call ffi::g_unix_mount_get_fs_type() }
//}

//#[cfg(any(unix, feature = "dox"))]
//pub fn unix_mount_get_mount_path(mount_entry: /*Ignored*/&mut UnixMountEntry) -> Option<std::path::PathBuf> {
//    unsafe { TODO: call ffi::g_unix_mount_get_mount_path() }
//}

//#[cfg(any(unix, feature = "dox"))]
//pub fn unix_mount_guess_can_eject(mount_entry: /*Ignored*/&mut UnixMountEntry) -> bool {
//    unsafe { TODO: call ffi::g_unix_mount_guess_can_eject() }
//}

//#[cfg(any(unix, feature = "dox"))]
//pub fn unix_mount_guess_icon(mount_entry: /*Ignored*/&mut UnixMountEntry) -> Option<Icon> {
//    unsafe { TODO: call ffi::g_unix_mount_guess_icon() }
//}

//#[cfg(any(unix, feature = "dox"))]
//pub fn unix_mount_guess_name(mount_entry: /*Ignored*/&mut UnixMountEntry) -> Option<String> {
//    unsafe { TODO: call ffi::g_unix_mount_guess_name() }
//}

//#[cfg(any(unix, feature = "dox"))]
//pub fn unix_mount_guess_should_display(mount_entry: /*Ignored*/&mut UnixMountEntry) -> bool {
//    unsafe { TODO: call ffi::g_unix_mount_guess_should_display() }
//}

//#[cfg(any(unix, feature = "dox"))]
//#[cfg(any(feature = "v2_34", feature = "dox"))]
//pub fn unix_mount_guess_symbolic_icon(mount_entry: /*Ignored*/&mut UnixMountEntry) -> Option<Icon> {
//    unsafe { TODO: call ffi::g_unix_mount_guess_symbolic_icon() }
//}

//#[cfg(any(unix, feature = "dox"))]
//pub fn unix_mount_is_readonly(mount_entry: /*Ignored*/&mut UnixMountEntry) -> bool {
//    unsafe { TODO: call ffi::g_unix_mount_is_readonly() }
//}

//#[cfg(any(unix, feature = "dox"))]
//pub fn unix_mount_is_system_internal(mount_entry: /*Ignored*/&mut UnixMountEntry) -> bool {
//    unsafe { TODO: call ffi::g_unix_mount_is_system_internal() }
//}

#[cfg(any(unix, feature = "dox"))]
pub fn unix_mount_points_changed_since(time: u64) -> bool {
    unsafe {
        from_glib(ffi::g_unix_mount_points_changed_since(time))
    }
}

//#[cfg(any(unix, feature = "dox"))]
//pub fn unix_mount_points_get() -> (/*Ignored*/Vec<UnixMountPoint>, u64) {
//    unsafe { TODO: call ffi::g_unix_mount_points_get() }
//}

#[cfg(any(unix, feature = "dox"))]
pub fn unix_mounts_changed_since(time: u64) -> bool {
    unsafe {
        from_glib(ffi::g_unix_mounts_changed_since(time))
    }
}

//#[cfg(any(unix, feature = "dox"))]
//pub fn unix_mounts_get() -> (/*Ignored*/Vec<UnixMountEntry>, u64) {
//    unsafe { TODO: call ffi::g_unix_mounts_get() }
//}

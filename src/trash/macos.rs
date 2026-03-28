use std::ffi::c_void;
use std::path::Path;
use std::path::PathBuf;

use crate::error::Result;
use crate::error::TrashError;

pub(crate) fn move_files_to_trash(paths: &[PathBuf]) -> Result<()> {
    // Build the direct-object list (files/folders) that Finder should move to Trash.
    let list_descriptor = build_file_descriptor_list(paths)?;
    // Build a target descriptor that points to the Finder process.
    let target_descriptor = build_finder_target_descriptor()?;

    send_delete_event_and_check_reply(target_descriptor.as_ref(), list_descriptor.as_ref())
}

fn build_file_descriptor_list(
    paths: &[PathBuf],
) -> Result<objc2::rc::Retained<objc2_foundation::NSAppleEventDescriptor>> {
    use objc2_foundation::NSAppleEventDescriptor;

    let list_descr = NSAppleEventDescriptor::listDescriptor();

    for path in paths {
        let descr =
            path_to_file_url_descriptor(path).ok_or_else(|| TrashError::DescriptorBuild {
                detail: format!("file URL descriptor for {}", path.display()),
            })?;
        list_descr.insertDescriptor_atIndex(&descr, 1);
    }

    Ok(list_descr)
}

fn build_finder_target_descriptor()
-> Result<objc2::rc::Retained<objc2_foundation::NSAppleEventDescriptor>> {
    let finder_pid = get_finder_pid().ok_or(TrashError::FinderNotRunning)?;
    let finder_ptr: *const c_void = &finder_pid as *const u32 as *const c_void;

    target_descriptor_for_finder(finder_ptr).ok_or_else(|| TrashError::DescriptorBuild {
        detail: "target descriptor for Finder".to_string(),
    })
}

fn send_delete_event_and_check_reply(
    target_descriptor: &objc2_foundation::NSAppleEventDescriptor,
    list_descriptor: &objc2_foundation::NSAppleEventDescriptor,
) -> Result<()> {
    use objc2_core_services::AEGetParamDesc;
    use objc2_core_services::AppleEvent;
    use objc2_foundation::NSAppleEventDescriptor;

    // Send Finder the 'core'/'delo' Apple Event (delete -> move to Trash in Finder context).
    let event = NSAppleEventDescriptor::appleEventWithEventClass_eventID_targetDescriptor_returnID_transactionID(
        u32::from_be_bytes(*b"core"),
        u32::from_be_bytes(*b"delo"),
        Some(target_descriptor),
        objc2_core_services::kAutoGenerateReturnID as i16,
        objc2_core_services::kAnyTransactionID,
    );

    event.setDescriptor_forKeyword(list_descriptor, objc2_core_services::keyDirectObject);

    // Receive the reply event to verify Finder accepted and processed the request.
    let mut reply_event: AppleEvent = unsafe { std::mem::zeroed() };
    let reply_event = &mut reply_event as *mut AppleEvent;

    let send_err = unsafe {
        objc2_core_services::AESendMessage(
            event.aeDesc(),
            reply_event,
            objc2_core_services::kAEWaitReply as i32,
            objc2_core_services::kAEDefaultTimeout as i64,
        )
    };
    if send_err != 0 {
        return Err(TrashError::AppleEventSend { status: send_err });
    }

    let mut reply_desc = AEDescDefaultExt::default();
    let get_reply_err = unsafe {
        AEGetParamDesc(
            reply_event,
            objc2_core_services::keyDirectObject,
            objc2_core_services::typeWildCard,
            &mut reply_desc,
        )
    };
    if get_reply_err != 0 {
        return Err(TrashError::AppleEventReply {
            status: i32::from(get_reply_err),
        });
    }

    Ok(())
}

fn target_descriptor_for_finder(
    finder_ptr: *const c_void,
) -> Option<objc2::rc::Retained<objc2_foundation::NSAppleEventDescriptor>> {
    use objc2_foundation::NSAppleEventDescriptor;
    let byte_count = std::mem::size_of::<u32>();

    unsafe {
        NSAppleEventDescriptor::descriptorWithDescriptorType_bytes_length(
            objc2_core_services::typeKernelProcessID,
            finder_ptr,
            byte_count,
        )
    }
}

fn path_to_file_url_descriptor<P: AsRef<Path>>(
    path: P,
) -> Option<objc2::rc::Retained<objc2_foundation::NSAppleEventDescriptor>> {
    use objc2_foundation::NSAppleEventDescriptor;
    use objc2_foundation::NSString;

    let path_ref = path.as_ref();
    let ns_string = NSString::from_str(&path_ref.to_string_lossy());
    let file_url = objc2_foundation::NSURL::fileURLWithPath(&ns_string);
    let data = file_url
        .absoluteString()
        .and_then(|string| string.dataUsingEncoding(objc2_foundation::NSUTF8StringEncoding));

    NSAppleEventDescriptor::descriptorWithDescriptorType_data(
        objc2_core_services::typeFileURL,
        data.as_deref(),
    )
}

fn get_finder_pid() -> Option<u32> {
    let finder_appid = objc2_foundation::NSString::from_str("com.apple.finder");
    let apps = objc2_app_kit::NSWorkspace::sharedWorkspace().runningApplications();

    for app in apps {
        if let Some(indentifier) = app.bundleIdentifier()
            && indentifier.isEqualToString(&finder_appid)
        {
            return Some(app.processIdentifier() as u32);
        }
    }
    None
}

trait AEDescDefaultExt {
    fn default() -> Self;
}

impl AEDescDefaultExt for objc2_core_services::AEDesc {
    fn default() -> Self {
        objc2_core_services::AEDesc {
            descriptorType: 0,
            dataHandle: std::ptr::null_mut(),
        }
    }
}

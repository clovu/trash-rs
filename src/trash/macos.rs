use path_absolutize::Absolutize;
use std::ffi::c_void;
use std::path::Path;

use super::TrashHandler;

pub struct Trash;

impl TrashHandler for Trash {
    fn move_files_to_trash(paths: Vec<String>) -> crate::error::Result<()> {
        use anyhow::anyhow;
        use anyhow::bail;
        use objc2_core_services::AEGetParamDesc;
        use objc2_core_services::AppleEvent;
        use objc2_foundation::NSAppleEventDescriptor;

        if paths.is_empty() {
            return Ok(());
        }

        let list_descr = NSAppleEventDescriptor::listDescriptor();

        for file_path in paths {
            let abs_path = Path::new(&file_path)
                .absolutize()
                .map_err(|_| anyhow!("failed to resolve path: {}", file_path))?;

            if !abs_path.exists() {
                return Err(anyhow!("file not exists: {}", abs_path.display()));
            }

            let descr = path_to_file_url_descriptor(abs_path)
                .ok_or_else(|| anyhow!("failed to create file URL descriptor for {}", file_path))?;
            list_descr.insertDescriptor_atIndex(&descr, 1);
        }

        // generate the 'top-level' "delete" descriptor
        let finder_pid = get_finder_pid().ok_or_else(|| anyhow!("could not find Finder PID"))?;
        let finder_ptr: *const c_void = &finder_pid as *const u32 as *const c_void;
        let target_desc = target_descriptor_for_finder(finder_ptr)
            .ok_or_else(|| anyhow!("failed to create target descriptor for Finder"))?;

        let event = NSAppleEventDescriptor::appleEventWithEventClass_eventID_targetDescriptor_returnID_transactionID(
            u32::from_be_bytes(*b"core"),
            u32::from_be_bytes(*b"delo"),
            Some(target_desc.as_ref()),
            objc2_core_services::kAutoGenerateReturnID as i16,
            objc2_core_services::kAnyTransactionID,
        );

        event.setDescriptor_forKeyword(&list_descr, objc2_core_services::keyDirectObject);

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
            bail!("AESendMessage failed: {}", send_err);
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
            bail!("AEGetParamDesc failed: {}", get_reply_err);
        }

        Ok(())
    }
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

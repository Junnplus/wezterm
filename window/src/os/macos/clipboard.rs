use crate::macos::nsstring_to_str;
use anyhow::anyhow;
use cocoa::appkit::NSPasteboard;
use cocoa::base::*;
use cocoa::foundation::{NSArray, NSDictionary, NSString, NSURL};
use objc::{class, msg_send, sel, sel_impl};

pub struct Clipboard {
    pasteboard: id,
}

#[link(name = "AppKit", kind = "framework")]
extern "C" {}

impl Clipboard {
    pub fn new() -> Self {
        let pasteboard = unsafe { NSPasteboard::generalPasteboard(nil) };
        Clipboard { pasteboard }
    }

    unsafe fn read_for_class(&self, cls: &objc::runtime::Class) -> Option<id> {
        let classes = msg_send![class!(NSArray), arrayWithObject: cls];
        let options = NSDictionary::dictionary(nil);
        let ok = self
            .pasteboard
            .canReadObjectForClasses_options(classes, options);
        if ok {
            let objs = self
                .pasteboard
                .readObjectsForClasses_options(classes, options);
            Some(objs.objectAtIndex(0))
        } else {
            None
        }
    }

    pub fn read(&self) -> anyhow::Result<String> {
        unsafe {
            if let Some(url) = self.read_for_class(class!(NSURL)) {
                return Ok(nsstring_to_str(url.absoluteString()).to_owned());
            }
            if let Some(string) = self.read_for_class(class!(NSString)) {
                return Ok(nsstring_to_str(string).to_owned());
            }
        }
        return Err(anyhow!("pasteboard read returned empty"));
    }

    pub fn write(&mut self, data: String) -> anyhow::Result<()> {
        unsafe {
            let obj = NSString::alloc(nil).init_str(&data);
            self.pasteboard.clearContents();
            let success = self
                .pasteboard
                .writeObjects(NSArray::arrayWithObject(nil, obj));
            if success {
                return Ok(());
            } else {
                return Err(anyhow!("pasteboard write returned false"));
            }
        }
    }
}

use zed_extension_api as zed;

struct VeriPyExtension {
    // ... state
}

impl zed::Extension for VeriPyExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        todo!()
    }
}

zed::register_extension!(VeriPyExtension);

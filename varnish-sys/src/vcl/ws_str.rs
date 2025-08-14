use std::{ffi::CStr, marker::PhantomData};

use crate::{ffi::VCL_STRING, vcl::IntoVCL};

pub struct WsStr<'a> {
    inner: VCL_STRING,
    _phantom: PhantomData<&'a ()>,
}

impl WsStr<'_> {
    pub(crate) fn new(inner: VCL_STRING) -> Self {
        Self {
            inner,
            _phantom: PhantomData,
        }
    }

    pub unsafe fn into_vcl(self) -> VCL_STRING {
        self.inner
    }

    pub fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.inner.0.cast()) }
    }
}

impl IntoVCL<VCL_STRING> for WsStr<'_> {
    fn into_vcl(self, _ws: &mut super::Workspace) -> Result<VCL_STRING, super::VclError> {
        unsafe { Ok(self.into_vcl()) }
    }
}

#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IDontSupportEventSubscription(::windows::core::IUnknown);
impl IDontSupportEventSubscription {}
impl ::core::convert::From<IDontSupportEventSubscription> for ::windows::core::IUnknown {
    fn from(value: IDontSupportEventSubscription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDontSupportEventSubscription> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDontSupportEventSubscription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDontSupportEventSubscription> for ::windows::core::IUnknown {
    fn from(value: &IDontSupportEventSubscription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDontSupportEventSubscription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDontSupportEventSubscription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDontSupportEventSubscription {}
impl ::core::fmt::Debug for IDontSupportEventSubscription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDontSupportEventSubscription").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDontSupportEventSubscription {
    type Vtable = IDontSupportEventSubscription_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x784121f1_62a6_4b89_855f_d65f296de83a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDontSupportEventSubscription_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEnumEventObject(::windows::core::IUnknown);
impl IEnumEventObject {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumEventObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumEventObject>(result__)
    }
    pub unsafe fn Next(&self, ppinterface: &mut [::core::option::Option<::windows::core::IUnknown>], cretelem: &mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ppinterface.len() as _, ::core::mem::transmute(ppinterface.as_ptr()), ::core::mem::transmute(cretelem)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, cskipelem: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), cskipelem).ok()
    }
}
impl ::core::convert::From<IEnumEventObject> for ::windows::core::IUnknown {
    fn from(value: IEnumEventObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumEventObject> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumEventObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumEventObject> for ::windows::core::IUnknown {
    fn from(value: &IEnumEventObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumEventObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumEventObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumEventObject {}
impl ::core::fmt::Debug for IEnumEventObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumEventObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumEventObject {
    type Vtable = IEnumEventObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4a07d63_2e25_11d1_9964_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumEventObject_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, creqelem: u32, ppinterface: *mut *mut ::core::ffi::c_void, cretelem: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cskipelem: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventClass(::windows::core::IUnknown);
impl IEventClass {
    pub unsafe fn EventClassID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EventClassID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetEventClassID(&self, bstreventclassid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventClassID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstreventclassid)).ok()
    }
    pub unsafe fn EventClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EventClassName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetEventClassName(&self, bstreventclassname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventClassName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstreventclassname)).ok()
    }
    pub unsafe fn OwnerSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).OwnerSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetOwnerSID(&self, bstrownersid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOwnerSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrownersid)).ok()
    }
    pub unsafe fn FiringInterfaceID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FiringInterfaceID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetFiringInterfaceID(&self, bstrfiringinterfaceid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFiringInterfaceID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrfiringinterfaceid)).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Description)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrdescription)).ok()
    }
    pub unsafe fn CustomConfigCLSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CustomConfigCLSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetCustomConfigCLSID(&self, bstrcustomconfigclsid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCustomConfigCLSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrcustomconfigclsid)).ok()
    }
    pub unsafe fn TypeLib(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TypeLib)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetTypeLib(&self, bstrtypelib: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTypeLib)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrtypelib)).ok()
    }
}
impl ::core::convert::From<IEventClass> for ::windows::core::IUnknown {
    fn from(value: IEventClass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEventClass> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEventClass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventClass> for ::windows::core::IUnknown {
    fn from(value: &IEventClass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IEventClass> for super::IDispatch {
    fn from(value: IEventClass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEventClass> for &'a super::IDispatch {
    fn from(value: &'a IEventClass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventClass> for super::IDispatch {
    fn from(value: &IEventClass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEventClass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventClass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventClass {}
impl ::core::fmt::Debug for IEventClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventClass").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEventClass {
    type Vtable = IEventClass_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb2b72a0_7a68_11d1_88f9_0080c7d771bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventClass_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub EventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetEventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstreventclassid: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub EventClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstreventclassname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetEventClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstreventclassname: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub OwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrownersid: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrownersid: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub FiringInterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfiringinterfaceid: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetFiringInterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfiringinterfaceid: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub CustomConfigCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcustomconfigclsid: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetCustomConfigCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcustomconfigclsid: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub TypeLib: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtypelib: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetTypeLib: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtypelib: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventClass2(::windows::core::IUnknown);
impl IEventClass2 {
    pub unsafe fn EventClassID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.EventClassID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetEventClassID(&self, bstreventclassid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEventClassID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstreventclassid)).ok()
    }
    pub unsafe fn EventClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.EventClassName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetEventClassName(&self, bstreventclassname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEventClassName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstreventclassname)).ok()
    }
    pub unsafe fn OwnerSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.OwnerSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetOwnerSID(&self, bstrownersid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetOwnerSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrownersid)).ok()
    }
    pub unsafe fn FiringInterfaceID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.FiringInterfaceID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetFiringInterfaceID(&self, bstrfiringinterfaceid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetFiringInterfaceID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrfiringinterfaceid)).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Description)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrdescription)).ok()
    }
    pub unsafe fn CustomConfigCLSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CustomConfigCLSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetCustomConfigCLSID(&self, bstrcustomconfigclsid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetCustomConfigCLSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrcustomconfigclsid)).ok()
    }
    pub unsafe fn TypeLib(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.TypeLib)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetTypeLib(&self, bstrtypelib: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetTypeLib)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrtypelib)).ok()
    }
    pub unsafe fn PublisherID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PublisherID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetPublisherID(&self, bstrpublisherid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPublisherID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrpublisherid)).ok()
    }
    pub unsafe fn MultiInterfacePublisherFilterCLSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MultiInterfacePublisherFilterCLSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetMultiInterfacePublisherFilterCLSID(&self, bstrpubfilclsid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMultiInterfacePublisherFilterCLSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrpubfilclsid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowInprocActivation(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AllowInprocActivation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowInprocActivation<'a, P0>(&self, fallowinprocactivation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetAllowInprocActivation)(::windows::core::Interface::as_raw(self), fallowinprocactivation.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FireInParallel(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FireInParallel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFireInParallel<'a, P0>(&self, ffireinparallel: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetFireInParallel)(::windows::core::Interface::as_raw(self), ffireinparallel.into()).ok()
    }
}
impl ::core::convert::From<IEventClass2> for ::windows::core::IUnknown {
    fn from(value: IEventClass2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEventClass2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEventClass2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventClass2> for ::windows::core::IUnknown {
    fn from(value: &IEventClass2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IEventClass2> for super::IDispatch {
    fn from(value: IEventClass2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEventClass2> for &'a super::IDispatch {
    fn from(value: &'a IEventClass2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventClass2> for super::IDispatch {
    fn from(value: &IEventClass2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IEventClass2> for IEventClass {
    fn from(value: IEventClass2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEventClass2> for &'a IEventClass {
    fn from(value: &'a IEventClass2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventClass2> for IEventClass {
    fn from(value: &IEventClass2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEventClass2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventClass2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventClass2 {}
impl ::core::fmt::Debug for IEventClass2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventClass2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEventClass2 {
    type Vtable = IEventClass2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb2b72a1_7a68_11d1_88f9_0080c7d771bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventClass2_Vtbl {
    pub base__: IEventClass_Vtbl,
    pub PublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublisherid: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub MultiInterfacePublisherFilterCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpubfilclsid: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetMultiInterfacePublisherFilterCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpubfilclsid: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FireInParallel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FireInParallel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFireInParallel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFireInParallel: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventControl(::windows::core::IUnknown);
impl IEventControl {
    pub unsafe fn SetPublisherFilter<'a, P0>(&self, methodname: &::windows::core::BSTR, ppublisherfilter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IPublisherFilter>>,
    {
        (::windows::core::Interface::vtable(self).SetPublisherFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(methodname), ppublisherfilter.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowInprocActivation(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AllowInprocActivation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowInprocActivation<'a, P0>(&self, fallowinprocactivation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetAllowInprocActivation)(::windows::core::Interface::as_raw(self), fallowinprocactivation.into()).ok()
    }
    pub unsafe fn GetSubscriptions(&self, methodname: &::windows::core::BSTR, optionalcriteria: &::windows::core::BSTR, optionalerrorindex: &i32) -> ::windows::core::Result<IEventObjectCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSubscriptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(methodname), ::core::mem::transmute_copy(optionalcriteria), ::core::mem::transmute(optionalerrorindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEventObjectCollection>(result__)
    }
    pub unsafe fn SetDefaultQuery(&self, methodname: &::windows::core::BSTR, criteria: &::windows::core::BSTR) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SetDefaultQuery)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(methodname), ::core::mem::transmute_copy(criteria), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
impl ::core::convert::From<IEventControl> for ::windows::core::IUnknown {
    fn from(value: IEventControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEventControl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEventControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventControl> for ::windows::core::IUnknown {
    fn from(value: &IEventControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IEventControl> for super::IDispatch {
    fn from(value: IEventControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEventControl> for &'a super::IDispatch {
    fn from(value: &'a IEventControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventControl> for super::IDispatch {
    fn from(value: &IEventControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEventControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventControl {}
impl ::core::fmt::Debug for IEventControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEventControl {
    type Vtable = IEventControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0343e2f4_86f6_11d1_b760_00c04fb926af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventControl_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SetPublisherFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, ppublisherfilter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowInprocActivation: usize,
    pub GetSubscriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, optionalcriteria: ::core::mem::ManuallyDrop<::windows::core::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDefaultQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, criteria: ::core::mem::ManuallyDrop<::windows::core::BSTR>, errorindex: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventObjectChange(::windows::core::IUnknown);
impl IEventObjectChange {
    pub unsafe fn ChangedSubscription(&self, changetype: EOC_ChangeType, bstrsubscriptionid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChangedSubscription)(::windows::core::Interface::as_raw(self), changetype, ::core::mem::transmute_copy(bstrsubscriptionid)).ok()
    }
    pub unsafe fn ChangedEventClass(&self, changetype: EOC_ChangeType, bstreventclassid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChangedEventClass)(::windows::core::Interface::as_raw(self), changetype, ::core::mem::transmute_copy(bstreventclassid)).ok()
    }
    pub unsafe fn ChangedPublisher(&self, changetype: EOC_ChangeType, bstrpublisherid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChangedPublisher)(::windows::core::Interface::as_raw(self), changetype, ::core::mem::transmute_copy(bstrpublisherid)).ok()
    }
}
impl ::core::convert::From<IEventObjectChange> for ::windows::core::IUnknown {
    fn from(value: IEventObjectChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEventObjectChange> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEventObjectChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventObjectChange> for ::windows::core::IUnknown {
    fn from(value: &IEventObjectChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEventObjectChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventObjectChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventObjectChange {}
impl ::core::fmt::Debug for IEventObjectChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventObjectChange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEventObjectChange {
    type Vtable = IEventObjectChange_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4a07d70_2e25_11d1_9964_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventObjectChange_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ChangedSubscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstrsubscriptionid: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ChangedEventClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstreventclassid: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ChangedPublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstrpublisherid: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventObjectChange2(::windows::core::IUnknown);
impl IEventObjectChange2 {
    pub unsafe fn ChangedSubscription(&self, pinfo: &COMEVENTSYSCHANGEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChangedSubscription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo)).ok()
    }
    pub unsafe fn ChangedEventClass(&self, pinfo: &COMEVENTSYSCHANGEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChangedEventClass)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo)).ok()
    }
}
impl ::core::convert::From<IEventObjectChange2> for ::windows::core::IUnknown {
    fn from(value: IEventObjectChange2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEventObjectChange2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEventObjectChange2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventObjectChange2> for ::windows::core::IUnknown {
    fn from(value: &IEventObjectChange2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEventObjectChange2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventObjectChange2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventObjectChange2 {}
impl ::core::fmt::Debug for IEventObjectChange2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventObjectChange2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEventObjectChange2 {
    type Vtable = IEventObjectChange2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7701a9c3_bd68_438f_83e0_67bf4f53a422);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventObjectChange2_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ChangedSubscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::HRESULT,
    pub ChangedEventClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventObjectCollection(::windows::core::IUnknown);
impl IEventObjectCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, objectid: &::windows::core::BSTR) -> ::windows::core::Result<super::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(objectid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::VARIANT>(result__)
    }
    pub unsafe fn NewEnum(&self) -> ::windows::core::Result<IEnumEventObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumEventObject>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Add(&self, item: &super::VARIANT, objectid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(item), ::core::mem::transmute_copy(objectid)).ok()
    }
    pub unsafe fn Remove(&self, objectid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(objectid)).ok()
    }
}
impl ::core::convert::From<IEventObjectCollection> for ::windows::core::IUnknown {
    fn from(value: IEventObjectCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEventObjectCollection> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEventObjectCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventObjectCollection> for ::windows::core::IUnknown {
    fn from(value: &IEventObjectCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IEventObjectCollection> for super::IDispatch {
    fn from(value: IEventObjectCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEventObjectCollection> for &'a super::IDispatch {
    fn from(value: &'a IEventObjectCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventObjectCollection> for super::IDispatch {
    fn from(value: &IEventObjectCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEventObjectCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventObjectCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventObjectCollection {}
impl ::core::fmt::Debug for IEventObjectCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventObjectCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEventObjectCollection {
    type Vtable = IEventObjectCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf89ac270_d4eb_11d1_b682_00805fc79216);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventObjectCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunkenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pitem: *mut super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *const super::VARIANT, objectid: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventProperty(::windows::core::IUnknown);
impl IEventProperty {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetName(&self, propertyname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(propertyname)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Value)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue(&self, propertyvalue: &super::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(propertyvalue)).ok()
    }
}
impl ::core::convert::From<IEventProperty> for ::windows::core::IUnknown {
    fn from(value: IEventProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEventProperty> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEventProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventProperty> for ::windows::core::IUnknown {
    fn from(value: &IEventProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IEventProperty> for super::IDispatch {
    fn from(value: IEventProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEventProperty> for &'a super::IDispatch {
    fn from(value: &'a IEventProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventProperty> for super::IDispatch {
    fn from(value: &IEventProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEventProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventProperty {}
impl ::core::fmt::Debug for IEventProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEventProperty {
    type Vtable = IEventProperty_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda538ee2_f4de_11d1_b6bb_00805fc79216);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventProperty_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    SetValue: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventPublisher(::windows::core::IUnknown);
impl IEventPublisher {
    pub unsafe fn PublisherID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PublisherID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetPublisherID(&self, bstrpublisherid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPublisherID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrpublisherid)).ok()
    }
    pub unsafe fn PublisherName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PublisherName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetPublisherName(&self, bstrpublishername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPublisherName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrpublishername)).ok()
    }
    pub unsafe fn PublisherType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PublisherType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetPublisherType(&self, bstrpublishertype: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPublisherType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrpublishertype)).ok()
    }
    pub unsafe fn OwnerSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).OwnerSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetOwnerSID(&self, bstrownersid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOwnerSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrownersid)).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Description)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrdescription)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetDefaultProperty(&self, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<super::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDefaultProperty)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn PutDefaultProperty(&self, bstrpropertyname: &::windows::core::BSTR, propertyvalue: &super::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PutDefaultProperty)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname), ::core::mem::transmute(propertyvalue)).ok()
    }
    pub unsafe fn RemoveDefaultProperty(&self, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveDefaultProperty)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname)).ok()
    }
    pub unsafe fn GetDefaultPropertyCollection(&self) -> ::windows::core::Result<IEventObjectCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDefaultPropertyCollection)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEventObjectCollection>(result__)
    }
}
impl ::core::convert::From<IEventPublisher> for ::windows::core::IUnknown {
    fn from(value: IEventPublisher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEventPublisher> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEventPublisher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventPublisher> for ::windows::core::IUnknown {
    fn from(value: &IEventPublisher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IEventPublisher> for super::IDispatch {
    fn from(value: IEventPublisher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEventPublisher> for &'a super::IDispatch {
    fn from(value: &'a IEventPublisher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventPublisher> for super::IDispatch {
    fn from(value: &IEventPublisher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEventPublisher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventPublisher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventPublisher {}
impl ::core::fmt::Debug for IEventPublisher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventPublisher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEventPublisher {
    type Vtable = IEventPublisher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe341516b_2e32_11d1_9964_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventPublisher_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub PublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublisherid: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub PublisherName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublishername: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetPublisherName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublishername: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub PublisherType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublishertype: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetPublisherType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublishertype: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub OwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrownersid: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrownersid: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetDefaultProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetDefaultProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub PutDefaultProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    PutDefaultProperty: usize,
    pub RemoveDefaultProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetDefaultPropertyCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventSubscription(::windows::core::IUnknown);
impl IEventSubscription {
    pub unsafe fn SubscriptionID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SubscriptionID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetSubscriptionID(&self, bstrsubscriptionid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSubscriptionID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrsubscriptionid)).ok()
    }
    pub unsafe fn SubscriptionName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SubscriptionName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetSubscriptionName(&self, bstrsubscriptionname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSubscriptionName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrsubscriptionname)).ok()
    }
    pub unsafe fn PublisherID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PublisherID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetPublisherID(&self, bstrpublisherid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPublisherID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrpublisherid)).ok()
    }
    pub unsafe fn EventClassID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EventClassID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetEventClassID(&self, bstreventclassid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventClassID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstreventclassid)).ok()
    }
    pub unsafe fn MethodName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MethodName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetMethodName(&self, bstrmethodname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMethodName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrmethodname)).ok()
    }
    pub unsafe fn SubscriberCLSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SubscriberCLSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetSubscriberCLSID(&self, bstrsubscriberclsid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSubscriberCLSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrsubscriberclsid)).ok()
    }
    pub unsafe fn SubscriberInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SubscriberInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn SetSubscriberInterface<'a, P0>(&self, psubscriberinterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).SetSubscriberInterface)(::windows::core::Interface::as_raw(self), psubscriberinterface.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PerUser(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PerUser)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPerUser<'a, P0>(&self, fperuser: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetPerUser)(::windows::core::Interface::as_raw(self), fperuser.into()).ok()
    }
    pub unsafe fn OwnerSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).OwnerSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetOwnerSID(&self, bstrownersid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOwnerSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrownersid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Enabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<'a, P0>(&self, fenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetEnabled)(::windows::core::Interface::as_raw(self), fenabled.into()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Description)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrdescription)).ok()
    }
    pub unsafe fn MachineName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MachineName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetMachineName(&self, bstrmachinename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMachineName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrmachinename)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetPublisherProperty(&self, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<super::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPublisherProperty)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn PutPublisherProperty(&self, bstrpropertyname: &::windows::core::BSTR, propertyvalue: &super::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PutPublisherProperty)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname), ::core::mem::transmute(propertyvalue)).ok()
    }
    pub unsafe fn RemovePublisherProperty(&self, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemovePublisherProperty)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname)).ok()
    }
    pub unsafe fn GetPublisherPropertyCollection(&self) -> ::windows::core::Result<IEventObjectCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPublisherPropertyCollection)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEventObjectCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetSubscriberProperty(&self, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<super::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSubscriberProperty)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn PutSubscriberProperty(&self, bstrpropertyname: &::windows::core::BSTR, propertyvalue: &super::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PutSubscriberProperty)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname), ::core::mem::transmute(propertyvalue)).ok()
    }
    pub unsafe fn RemoveSubscriberProperty(&self, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveSubscriberProperty)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname)).ok()
    }
    pub unsafe fn GetSubscriberPropertyCollection(&self) -> ::windows::core::Result<IEventObjectCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSubscriberPropertyCollection)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEventObjectCollection>(result__)
    }
    pub unsafe fn InterfaceID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).InterfaceID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetInterfaceID(&self, bstrinterfaceid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInterfaceID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(bstrinterfaceid)).ok()
    }
}
impl ::core::convert::From<IEventSubscription> for ::windows::core::IUnknown {
    fn from(value: IEventSubscription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEventSubscription> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEventSubscription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventSubscription> for ::windows::core::IUnknown {
    fn from(value: &IEventSubscription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IEventSubscription> for super::IDispatch {
    fn from(value: IEventSubscription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEventSubscription> for &'a super::IDispatch {
    fn from(value: &'a IEventSubscription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventSubscription> for super::IDispatch {
    fn from(value: &IEventSubscription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEventSubscription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventSubscription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventSubscription {}
impl ::core::fmt::Debug for IEventSubscription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventSubscription").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEventSubscription {
    type Vtable = IEventSubscription_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a6b0e15_2e38_11d1_9965_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventSubscription_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SubscriptionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubscriptionid: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetSubscriptionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubscriptionid: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SubscriptionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubscriptionname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetSubscriptionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubscriptionname: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub PublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublisherid: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub EventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetEventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstreventclassid: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub MethodName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmethodname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetMethodName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmethodname: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SubscriberCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubscriberclsid: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetSubscriberCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubscriberclsid: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SubscriberInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsubscriberinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSubscriberInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psubscriberinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PerUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfperuser: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PerUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPerUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fperuser: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPerUser: usize,
    pub OwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrownersid: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrownersid: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenabled: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub MachineName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmachinename: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetMachineName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmachinename: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetPublisherProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetPublisherProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub PutPublisherProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    PutPublisherProperty: usize,
    pub RemovePublisherProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetPublisherPropertyCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetSubscriberProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetSubscriberProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub PutSubscriberProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    PutSubscriberProperty: usize,
    pub RemoveSubscriberProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetSubscriberPropertyCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrinterfaceid: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetInterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinterfaceid: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventSystem(::windows::core::IUnknown);
impl IEventSystem {
    pub unsafe fn Query(&self, progid: &::windows::core::BSTR, querycriteria: &::windows::core::BSTR, errorindex: &mut i32, ppinterface: &mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Query)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(progid), ::core::mem::transmute_copy(querycriteria), ::core::mem::transmute(errorindex), ::core::mem::transmute(ppinterface)).ok()
    }
    pub unsafe fn Store<'a, P0>(&self, progid: &::windows::core::BSTR, pinterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).Store)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(progid), pinterface.into().abi()).ok()
    }
    pub unsafe fn Remove(&self, progid: &::windows::core::BSTR, querycriteria: &::windows::core::BSTR) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(progid), ::core::mem::transmute_copy(querycriteria), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn EventObjectChangeEventClassID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EventObjectChangeEventClassID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn QueryS(&self, progid: &::windows::core::BSTR, querycriteria: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueryS)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(progid), ::core::mem::transmute_copy(querycriteria), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn RemoveS(&self, progid: &::windows::core::BSTR, querycriteria: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveS)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(progid), ::core::mem::transmute_copy(querycriteria)).ok()
    }
}
impl ::core::convert::From<IEventSystem> for ::windows::core::IUnknown {
    fn from(value: IEventSystem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEventSystem> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEventSystem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventSystem> for ::windows::core::IUnknown {
    fn from(value: &IEventSystem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IEventSystem> for super::IDispatch {
    fn from(value: IEventSystem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEventSystem> for &'a super::IDispatch {
    fn from(value: &'a IEventSystem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventSystem> for super::IDispatch {
    fn from(value: &IEventSystem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEventSystem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventSystem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventSystem {}
impl ::core::fmt::Debug for IEventSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventSystem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEventSystem {
    type Vtable = IEventSystem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e14fb9f_2e22_11d1_9964_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventSystem_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<::windows::core::BSTR>, querycriteria: ::core::mem::ManuallyDrop<::windows::core::BSTR>, errorindex: *mut i32, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Store: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<::windows::core::BSTR>, pinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<::windows::core::BSTR>, querycriteria: ::core::mem::ManuallyDrop<::windows::core::BSTR>, errorindex: *mut i32) -> ::windows::core::HRESULT,
    pub EventObjectChangeEventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub QueryS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<::windows::core::BSTR>, querycriteria: ::core::mem::ManuallyDrop<::windows::core::BSTR>, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<::windows::core::BSTR>, querycriteria: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IFiringControl(::windows::core::IUnknown);
impl IFiringControl {
    pub unsafe fn FireSubscription<'a, P0>(&self, subscription: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IEventSubscription>>,
    {
        (::windows::core::Interface::vtable(self).FireSubscription)(::windows::core::Interface::as_raw(self), subscription.into().abi()).ok()
    }
}
impl ::core::convert::From<IFiringControl> for ::windows::core::IUnknown {
    fn from(value: IFiringControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFiringControl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFiringControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFiringControl> for ::windows::core::IUnknown {
    fn from(value: &IFiringControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IFiringControl> for super::IDispatch {
    fn from(value: IFiringControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFiringControl> for &'a super::IDispatch {
    fn from(value: &'a IFiringControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFiringControl> for super::IDispatch {
    fn from(value: &IFiringControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IFiringControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFiringControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFiringControl {}
impl ::core::fmt::Debug for IFiringControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFiringControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFiringControl {
    type Vtable = IFiringControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0498c93_4efe_11d1_9971_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFiringControl_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub FireSubscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subscription: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IMultiInterfaceEventControl(::windows::core::IUnknown);
impl IMultiInterfaceEventControl {
    pub unsafe fn SetMultiInterfacePublisherFilter<'a, P0>(&self, classfilter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMultiInterfacePublisherFilter>>,
    {
        (::windows::core::Interface::vtable(self).SetMultiInterfacePublisherFilter)(::windows::core::Interface::as_raw(self), classfilter.into().abi()).ok()
    }
    pub unsafe fn GetSubscriptions(&self, eventiid: &::windows::core::GUID, bstrmethodname: &::windows::core::BSTR, optionalcriteria: &::windows::core::BSTR, optionalerrorindex: &i32) -> ::windows::core::Result<IEventObjectCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSubscriptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(eventiid), ::core::mem::transmute_copy(bstrmethodname), ::core::mem::transmute_copy(optionalcriteria), ::core::mem::transmute(optionalerrorindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEventObjectCollection>(result__)
    }
    pub unsafe fn SetDefaultQuery(&self, eventiid: &::windows::core::GUID, bstrmethodname: &::windows::core::BSTR, bstrcriteria: &::windows::core::BSTR) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SetDefaultQuery)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(eventiid), ::core::mem::transmute_copy(bstrmethodname), ::core::mem::transmute_copy(bstrcriteria), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowInprocActivation(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AllowInprocActivation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowInprocActivation<'a, P0>(&self, fallowinprocactivation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetAllowInprocActivation)(::windows::core::Interface::as_raw(self), fallowinprocactivation.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FireInParallel(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FireInParallel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFireInParallel<'a, P0>(&self, ffireinparallel: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetFireInParallel)(::windows::core::Interface::as_raw(self), ffireinparallel.into()).ok()
    }
}
impl ::core::convert::From<IMultiInterfaceEventControl> for ::windows::core::IUnknown {
    fn from(value: IMultiInterfaceEventControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMultiInterfaceEventControl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMultiInterfaceEventControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMultiInterfaceEventControl> for ::windows::core::IUnknown {
    fn from(value: &IMultiInterfaceEventControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMultiInterfaceEventControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMultiInterfaceEventControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMultiInterfaceEventControl {}
impl ::core::fmt::Debug for IMultiInterfaceEventControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultiInterfaceEventControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMultiInterfaceEventControl {
    type Vtable = IMultiInterfaceEventControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0343e2f5_86f6_11d1_b760_00c04fb926af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiInterfaceEventControl_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetMultiInterfacePublisherFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classfilter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSubscriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventiid: *const ::windows::core::GUID, bstrmethodname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, optionalcriteria: ::core::mem::ManuallyDrop<::windows::core::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDefaultQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventiid: *const ::windows::core::GUID, bstrmethodname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, bstrcriteria: ::core::mem::ManuallyDrop<::windows::core::BSTR>, errorindex: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FireInParallel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FireInParallel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFireInParallel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFireInParallel: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IMultiInterfacePublisherFilter(::windows::core::IUnknown);
impl IMultiInterfacePublisherFilter {
    pub unsafe fn Initialize<'a, P0>(&self, peic: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMultiInterfaceEventControl>>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), peic.into().abi()).ok()
    }
    pub unsafe fn PrepareToFire<'a, P0>(&self, iid: &::windows::core::GUID, methodname: &::windows::core::BSTR, firingcontrol: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFiringControl>>,
    {
        (::windows::core::Interface::vtable(self).PrepareToFire)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(iid), ::core::mem::transmute_copy(methodname), firingcontrol.into().abi()).ok()
    }
}
impl ::core::convert::From<IMultiInterfacePublisherFilter> for ::windows::core::IUnknown {
    fn from(value: IMultiInterfacePublisherFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMultiInterfacePublisherFilter> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMultiInterfacePublisherFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMultiInterfacePublisherFilter> for ::windows::core::IUnknown {
    fn from(value: &IMultiInterfacePublisherFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMultiInterfacePublisherFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMultiInterfacePublisherFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMultiInterfacePublisherFilter {}
impl ::core::fmt::Debug for IMultiInterfacePublisherFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultiInterfacePublisherFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMultiInterfacePublisherFilter {
    type Vtable = IMultiInterfacePublisherFilter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x465e5cc1_7b26_11d1_88fb_0080c7d771bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiInterfacePublisherFilter_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peic: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PrepareToFire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, methodname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, firingcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IPublisherFilter(::windows::core::IUnknown);
impl IPublisherFilter {
    pub unsafe fn Initialize<'a, P0>(&self, methodname: &::windows::core::BSTR, dispuserdefined: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IDispatch>>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(methodname), dispuserdefined.into().abi()).ok()
    }
    pub unsafe fn PrepareToFire<'a, P0>(&self, methodname: &::windows::core::BSTR, firingcontrol: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFiringControl>>,
    {
        (::windows::core::Interface::vtable(self).PrepareToFire)(::windows::core::Interface::as_raw(self), ::core::mem::transmute_copy(methodname), firingcontrol.into().abi()).ok()
    }
}
impl ::core::convert::From<IPublisherFilter> for ::windows::core::IUnknown {
    fn from(value: IPublisherFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPublisherFilter> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPublisherFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPublisherFilter> for ::windows::core::IUnknown {
    fn from(value: &IPublisherFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPublisherFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPublisherFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPublisherFilter {}
impl ::core::fmt::Debug for IPublisherFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPublisherFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPublisherFilter {
    type Vtable = IPublisherFilter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x465e5cc0_7b26_11d1_88fb_0080c7d771bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPublisherFilter_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, dispuserdefined: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PrepareToFire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, firingcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
pub const CEventClass: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdbec9c0_7a68_11d1_88f9_0080c7d771bf);
pub const CEventPublisher: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab944620_79c6_11d1_88f9_0080c7d771bf);
pub const CEventSubscription: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7542e960_79c7_11d1_88f9_0080c7d771bf);
pub const CEventSystem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e14fba2_2e22_11d1_9964_00c04fbbb345);
pub const EventObjectChange: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0565000_9df4_11d1_a281_00c04fca0aa7);
pub const EventObjectChange2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb07bacd_cd56_4e63_a8ff_cbf0355fb9f4);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EOC_ChangeType(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EOC_NewObject: EOC_ChangeType = EOC_ChangeType(0i32);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EOC_ModifiedObject: EOC_ChangeType = EOC_ChangeType(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EOC_DeletedObject: EOC_ChangeType = EOC_ChangeType(2i32);
impl ::core::marker::Copy for EOC_ChangeType {}
impl ::core::clone::Clone for EOC_ChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EOC_ChangeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EOC_ChangeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for EOC_ChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EOC_ChangeType").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub struct COMEVENTSYSCHANGEINFO {
    pub cbSize: u32,
    pub changeType: EOC_ChangeType,
    pub objectId: ::windows::core::BSTR,
    pub partitionId: ::windows::core::BSTR,
    pub applicationId: ::windows::core::BSTR,
    pub reserved: [::windows::core::GUID; 10],
}
impl ::core::clone::Clone for COMEVENTSYSCHANGEINFO {
    fn clone(&self) -> Self {
        Self {
            cbSize: self.cbSize,
            changeType: self.changeType,
            objectId: self.objectId.clone(),
            partitionId: self.partitionId.clone(),
            applicationId: self.applicationId.clone(),
            reserved: self.reserved,
        }
    }
}
impl ::core::fmt::Debug for COMEVENTSYSCHANGEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMEVENTSYSCHANGEINFO").field("cbSize", &self.cbSize).field("changeType", &self.changeType).field("objectId", &self.objectId).field("partitionId", &self.partitionId).field("applicationId", &self.applicationId).field("reserved", &self.reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for COMEVENTSYSCHANGEINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for COMEVENTSYSCHANGEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.changeType == other.changeType && self.objectId == other.objectId && self.partitionId == other.partitionId && self.applicationId == other.applicationId && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for COMEVENTSYSCHANGEINFO {}
impl ::core::default::Default for COMEVENTSYSCHANGEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

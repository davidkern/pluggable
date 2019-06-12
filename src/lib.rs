pub extern crate pluggable_macros;

use std::mem;

pub use pluggable_macros::pluggable;

pub trait IID {
    const IID: u128;
}

// Opaque structure representing a component
#[repr(C)]
pub struct OpaqueComponent { _unused: [u8;0] }

// All Vtbl implement these functions
#[repr(C)]
pub struct IComponentVtbl {
    pub get_interface: fn(component: *const crate::OpaqueComponent, iid: u128) -> Option<IComponent>,
    pub increment_ref: fn(component: *const crate::OpaqueComponent),
    pub decrement_ref: fn(component: *const crate::OpaqueComponent),
}

impl IID for IComponentVtbl {
    const IID: u128 = 0x00000000_00000000_00000000_00000000;
}

impl AsRef<IComponentVtbl> for IComponentVtbl {
    fn as_ref(&self) -> &Self {
        self
    }
}

// Structure representing an interface
#[repr(C)]
pub struct Interface<TVtbl: 'static + AsRef<IComponentVtbl>> {
    pub component: *const crate::OpaqueComponent,
    pub vtbl: &'static TVtbl,
}

// pub type IComponent = Interface<EmptyVtbl>;
pub type IComponent = Interface<IComponentVtbl>;

impl IComponent {
    pub fn get_interface<TVtbl>(&self) -> Option<crate::Interface<TVtbl>>
    where TVtbl: IID + AsRef<IComponentVtbl>
    {
        // Cast an interface into a different interface. This is safe
        // to do since the vtbls must start with the same methods as defined
        // by IComponentVtbl, and the IID which selects the new vtbl
        // is bound to that vtbl.  The implementation macro enforces these
        // rules.
        unsafe {
            match (self.vtbl.get_interface)(&*self.component, TVtbl::IID) {
                Some(interface) => Some(mem::transmute(interface)),
                None => None,
            }
        }
    }
}

// Decrement component reference count when interface is dropped
impl<TVtbl> Drop for Interface<TVtbl> where TVtbl: AsRef<IComponentVtbl> {
    fn drop(&mut self) {
        (self.vtbl.as_ref().decrement_ref)(self.component);
    }
}

// Increment component reference count when interface is cloned
impl<TVtbl> Clone for Interface<TVtbl> where TVtbl: AsRef<IComponentVtbl> {
    fn clone(&self) -> Self {
        (self.vtbl.as_ref().increment_ref)(self.component);
        Interface {
            component: self.component,
            vtbl: self.vtbl,
        }
    }
}

use crate::{
    c::{
        spSkeletonData, spSkin, spSkin_addSkin, spSkin_copySkin, spSkin_create, spSkin_dispose,
        spSkin_getAttachments,
    },
    c_interface::{CTmpMut, CTmpRef, NewFromPtr, SyncPtr},
    Attachment, Skeleton, SkeletonData,
};
use std::ffi::CString;

/// A container for attachments which can be applied to a skeleton.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#Skin)
#[derive(Debug)]
pub struct Skin {
    c_skin: SyncPtr<spSkin>,
    pub(crate) owns_memory: bool,
}

impl NewFromPtr<spSkin> for Skin {
    unsafe fn new_from_ptr(c_skin: *const spSkin) -> Self {
        Self {
            c_skin: SyncPtr(c_skin as *mut spSkin),
            owns_memory: false,
        }
    }
}

impl Skin {
    pub fn new(name: &str) -> Skin {
        let c_name = CString::new(name).unwrap();
        Self {
            c_skin: SyncPtr(unsafe { spSkin_create(c_name.as_ptr()) }),
            owns_memory: true,
        }
    }

    pub fn add_skin(&mut self, other: &Skin) {
        unsafe {
            spSkin_addSkin(self.c_ptr_mut(), other.c_ptr());
        }
    }

    pub fn copy_skin(&mut self, other: &Skin) {
        unsafe {
            spSkin_copySkin(self.c_ptr_mut(), other.c_ptr());
        }
    }

    pub fn attachments(&self) -> Vec<AttachmentEntry> {
        let mut attachments = vec![];
        unsafe {
            let mut entry = spSkin_getAttachments(self.c_ptr());
            while !entry.is_null() {
                attachments.push(AttachmentEntry {
                    slot_index: (*entry).slotIndex,
                    attachment: Attachment::new_from_ptr((*entry).attachment),
                });
                entry = (*entry).next;
            }
        }
        attachments
    }

    c_accessor_string!(name, name);
    c_ptr!(c_skin, spSkin);
    // TODO: accessors
}

impl Clone for Skin {
    fn clone(&self) -> Self {
        let mut clone = Skin::new(self.name());
        clone.copy_skin(self);
        clone
    }
}

impl Drop for Skin {
    fn drop(&mut self) {
        if self.owns_memory {
            unsafe {
                spSkin_dispose(self.c_skin.0);
            }
        }
    }
}

c_handle_decl!(
    /// A storeable reference to a [`Skin`].
    ///
    /// Can be acquired from a
    /// [`CTmpRef<SkeletonData, Skin>`], [`CTmpMut<SkeletonData, Skin>`],
    /// [`CTmpRef<Skeleton, Skin>`], or [`CTmpMut<Skeleton, Skin>`].
    ///
    /// ```
    /// # #[path="./test.rs"]
    /// # mod test;
    /// # use rusty_spine::{AnimationState, EventType, SkinHandle};
    /// # let (skeleton, _) = test::TestAsset::spineboy().instance();
    /// let skeleton_data = skeleton.data();
    /// let skin_handles: Vec<SkinHandle> = skeleton_data.skins().map(|skin| skin.handle()).collect();
    /// for skin_handle in skin_handles.iter() {
    ///     let skin = skin_handle.get(skeleton_data.as_ref()).unwrap();
    ///     println!("{}", skin.name());
    /// }
    /// ```
    SkinHandle,
    Skin,
    SkeletonData,
    spSkin,
    spSkeletonData
);

impl<'a> CTmpRef<'a, SkeletonData, Skin> {
    pub fn handle(&self) -> SkinHandle {
        SkinHandle::new(self.c_ptr(), self.parent.c_ptr())
    }
}

impl<'a> CTmpMut<'a, SkeletonData, Skin> {
    pub fn handle(&self) -> SkinHandle {
        SkinHandle::new(self.c_ptr(), self.parent.c_ptr())
    }
}

impl<'a> CTmpRef<'a, Skeleton, Skin> {
    pub fn handle(&self) -> SkinHandle {
        SkinHandle::new(self.c_ptr(), unsafe { self.parent.c_ptr_mut().data })
    }
}

impl<'a> CTmpMut<'a, Skeleton, Skin> {
    pub fn handle(&self) -> SkinHandle {
        SkinHandle::new(self.c_ptr(), unsafe { self.parent.c_ptr_mut().data })
    }
}

pub struct AttachmentEntry {
    pub slot_index: i32,
    pub attachment: Attachment,
}

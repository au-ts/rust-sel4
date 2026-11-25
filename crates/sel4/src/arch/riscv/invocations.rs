//
// Copyright 2023, Colias Group, LLC
//
// SPDX-License-Identifier: MIT
//

use crate::{
    AbsoluteCPtr, Cap, CapRights, CapTypeForFrameObject, Error, InvocationContext, Result,
    TranslationTableObjectType, VmAttributes, Word, cap::*, cap_type,
};

impl<T: CapTypeForFrameObject, C: InvocationContext> Cap<T, C> {
    /// Corresponds to `seL4_RISCV_Page_Map`.
    pub fn frame_map(
        self,
        page_table: PageTable,
        vaddr: usize,
        rights: CapRights,
        attrs: VmAttributes,
    ) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer.inner_mut().seL4_RISCV_Page_Map(
                cptr.bits(),
                page_table.bits(),
                vaddr.try_into().unwrap(),
                rights.into_inner(),
                attrs.into_inner(),
            )
        }))
    }

    /// Corresponds to `seL4_RISCV_Page_Unmap`.
    pub fn frame_unmap(self) -> Result<()> {
        Error::wrap(
            self.invoke(|cptr, ipc_buffer| {
                ipc_buffer.inner_mut().seL4_RISCV_Page_Unmap(cptr.bits())
            }),
        )
    }

    /// Corresponds to `seL4_RISCV_Page_GetAddress`.
    pub fn frame_get_address(self) -> Result<usize> {
        let ret = self.invoke(|cptr, ipc_buffer| {
            ipc_buffer
                .inner_mut()
                .seL4_RISCV_Page_GetAddress(cptr.bits())
        });
        match Error::from_sys(ret.error) {
            None => Ok(ret.paddr.try_into().unwrap()),
            Some(err) => Err(err),
        }
    }
}

impl<C: InvocationContext> PageTable<C> {
    pub fn page_table_map(self, vspace: PageTable, vaddr: usize, attr: VmAttributes) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer.inner_mut().seL4_RISCV_PageTable_Map(
                cptr.bits(),
                vspace.bits(),
                vaddr.try_into().unwrap(),
                attr.into_inner(),
            )
        }))
    }
}

impl<C: InvocationContext> UnspecifiedIntermediateTranslationTable<C> {
    pub fn generic_intermediate_translation_table_map(
        self,
        _ty: TranslationTableObjectType,
        vspace: VSpace,
        vaddr: usize,
        attr: VmAttributes,
    ) -> Result<()> {
        self.cast::<cap_type::PageTable>()
            .page_table_map(vspace, vaddr, attr)
    }
}

impl<C: InvocationContext> IrqControl<C> {
    /// Corresponds to `seL4_IRQControl_GetTrigger`.
    #[sel4_config::sel4_cfg(not(RISCV_AIA))]
    pub fn irq_control_get_trigger(
        self,
        irq: Word,
        edge_triggered: bool,
        dst: &AbsoluteCPtr,
    ) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer.inner_mut().seL4_IRQControl_GetTrigger(
                cptr.bits(),
                irq,
                edge_triggered.into(),
                dst.root().bits(),
                dst.path().bits(),
                dst.path().depth_for_kernel(),
            )
        }))
    }
    #[sel4_config::sel4_cfg(RISCV_APLIC)]
    pub fn irq_control_get_aplic(
        self,
        irq: Word,
        source: Word,
        trigger: Word,
        dst: &AbsoluteCPtr,
    ) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer.inner_mut().seL4_IRQControl_GetAPLIC(
                cptr.bits(),
                irq,
                source,
                trigger,
                dst.root().bits(),
                dst.path().bits(),
                dst.path().depth_for_kernel(),
            )
        }))
    }
}

impl<C: InvocationContext> AsidControl<C> {
    /// Corresponds to `seL4_RISCV_ASIDControl_MakePool`.
    pub fn asid_control_make_pool(self, untyped: Untyped, dst: &AbsoluteCPtr) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer.inner_mut().seL4_RISCV_ASIDControl_MakePool(
                cptr.bits(),
                untyped.bits(),
                dst.root().bits(),
                dst.path().bits(),
                dst.path().depth_for_kernel(),
            )
        }))
    }
}

impl<C: InvocationContext> AsidPool<C> {
    /// Corresponds to `seL4_RISCV_ASIDPool_Assign`.
    pub fn asid_pool_assign(self, vspace: PageTable) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer
                .inner_mut()
                .seL4_RISCV_ASIDPool_Assign(cptr.bits(), vspace.bits())
        }))
    }
}

#[sel4_config::sel4_cfg(RISCV_IOMMU)]
impl<C: InvocationContext> IOMMUControl<C> {
    pub fn iommu_get_capabilities(self) -> Result<(Word, Word)> {
        let ret = self.invoke(|cptr, ipc_buffer| {
            ipc_buffer
                .inner_mut()
                .seL4_RISCV_IOMMUControl_GetCapabilities(cptr.bits())
        });
        Error::wrap(ret.error)?;
        Ok((ret.capabilities, ret.ddtLevels))
    }

    pub fn iommu_configure_fault_interrupt(self, enable: bool, identity: Word) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer
                .inner_mut()
                .seL4_RISCV_IOMMUControl_ConfigureFaultInterrupt(
                    cptr.bits(),
                    enable.into(),
                    identity,
                )
        }))
    }

    pub fn iommu_map_fault_queue(self, frame: Cap<impl CapTypeForFrameObject>) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer
                .inner_mut()
                .seL4_RISCV_IOMMUControl_MapFaultQueue(cptr.bits(), frame.bits())
        }))
    }

    pub fn iommu_rw_fault_queue(self, head: Word, clear_errors: Word) -> Result<(Word, Word)> {
        let ret = self.invoke(|cptr, ipc_buffer| {
            ipc_buffer.inner_mut().seL4_RISCV_IOMMUControl_RWFaultQueue(
                cptr.bits(),
                head,
                clear_errors,
            )
        });
        Error::wrap(ret.error)?;
        Ok((ret.tail, ret.status))
    }
}

#[sel4_config::sel4_cfg(RISCV_IOMMU)]
impl<C: InvocationContext> IOMMUDDT<C> {
    pub fn iommu_ddt_map(self, iommu: IOMMUControl, device_id: Word) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer
                .inner_mut()
                .seL4_RISCV_IOMMUDDT_Map(cptr.bits(), iommu.bits(), device_id)
        }))
    }

    pub fn iommu_ddt_unmap(self) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer
                .inner_mut()
                .seL4_RISCV_IOMMUDDT_Unmap(cptr.bits())
        }))
    }
}

#[sel4_config::sel4_cfg(RISCV_IOMMU)]
impl<C: InvocationContext> IOMMUDevice<C> {
    pub fn iommu_device_map(
        self,
        iommu: IOMMUControl,
        root: RootX4PageTable,
        device_id: Word,
    ) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer.inner_mut().seL4_RISCV_IOMMUDevice_Map(
                cptr.bits(),
                iommu.bits(),
                root.bits(),
                device_id,
            )
        }))
    }

    pub fn iommu_device_unmap(self) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer
                .inner_mut()
                .seL4_RISCV_IOMMUDevice_Unmap(cptr.bits())
        }))
    }

    pub fn iommu_device_configure_first_stage(
        self,
        kind: Word,
        mode: Word,
        root_gpa: usize,
        pscid: Word,
        flags: Word,
    ) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer
                .inner_mut()
                .seL4_RISCV_IOMMUDevice_ConfigureFirstStage(
                    cptr.bits(),
                    kind,
                    mode,
                    root_gpa.try_into().unwrap(),
                    pscid,
                    flags,
                )
        }))
    }

    pub fn iommu_device_invalidate_first_stage(self, kind: Word, pscid: Word) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer
                .inner_mut()
                .seL4_RISCV_IOMMUDevice_InvalidateFirstStage(cptr.bits(), kind, pscid)
        }))
    }
}

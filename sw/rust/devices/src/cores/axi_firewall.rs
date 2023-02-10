use crate::xdma::{BasedUserOps, Error as XdmaError};
use enum_iterator::Sequence;

const MI_BLOCK_MASK: u32 = 0x0100_0100;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    XdmaFailed(XdmaError),
}

/// AXI firewall registers
#[derive(Copy, Clone, Debug, Sequence, PartialEq)]
#[repr(u64)]
pub enum AxiFirewallReg {
    MiSideFaultStatus = 0x0,
    MiSideSoftFaultControl = 0x4,
    MiSideUnblockControl = 0x8,
    IpVersion = 0x10,
}

pub trait AxiFirewallOps {
    /// Reads the value of an AXI firewall register
    fn get_axi_firewall_reg(&self, reg: AxiFirewallReg) -> Result<u32>;

    /// Writes the value of an AXI firewall register
    fn set_axi_firewall_reg(&self, reg: AxiFirewallReg, value: u32) -> Result<()>;

    fn get_mi_fault_status(&self) -> Result<u32> {
        self.get_axi_firewall_reg(AxiFirewallReg::MiSideFaultStatus)
    }

    fn mi_is_blocked(&self) -> Result<bool> {
        Ok(self.get_mi_fault_status()? != 0)
    }

    fn block_mi(&self) -> Result<()> {
        self.set_axi_firewall_reg(AxiFirewallReg::MiSideSoftFaultControl, MI_BLOCK_MASK)
    }

    fn unblock_mi(&self) -> Result<()> {
        self.set_axi_firewall_reg(AxiFirewallReg::MiSideSoftFaultControl, 0)?;
        self.set_axi_firewall_reg(AxiFirewallReg::MiSideUnblockControl, 1)
    }

    fn get_ip_version(&self) -> Result<u32> {
        self.get_axi_firewall_reg(AxiFirewallReg::IpVersion)
    }
}

impl<T> AxiFirewallOps for T
where
    T: BasedUserOps,
{
    fn get_axi_firewall_reg(&self, reg: AxiFirewallReg) -> Result<u32> {
        self.based_user_read_u32(reg as u64)
            .map_err(Error::XdmaFailed)
    }

    fn set_axi_firewall_reg(&self, reg: AxiFirewallReg, value: u32) -> Result<()> {
        self.based_user_write_u32(reg as u64, value)
            .map_err(Error::XdmaFailed)
    }
}

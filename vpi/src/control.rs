#[repr(u32)]
pub enum Control {
    Stop = vpi_sys::vpiStop,
    Finish = vpi_sys::vpiFinish,
    Reset = vpi_sys::vpiReset,
    SetInteractiveScope = vpi_sys::vpiSetInteractiveScope,
}

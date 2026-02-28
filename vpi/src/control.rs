#[repr(u32)]
pub enum Control {
    Stop = vpi_sys::vpiStop,
    Finish = vpi_sys::vpiFinish,
    Reset = vpi_sys::vpiReset,
    SetInteractiveScope = vpi_sys::vpiSetInteractiveScope,
}

pub fn control(control: Control) {
    unsafe {
        vpi_sys::vpi_control(control as i32);
    }
}

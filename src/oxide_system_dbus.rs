// This code was autogenerated with `dbus-codegen-rust `, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus_tree as tree;

pub trait CodesEeemsOxide1System {
    fn suspend(&self) -> Result<(), tree::MethodErr>;
    fn power_off(&self) -> Result<(), tree::MethodErr>;
    fn reboot(&self) -> Result<(), tree::MethodErr>;
    fn activity(&self) -> Result<(), tree::MethodErr>;
    fn inhibit_sleep(&self) -> Result<(), tree::MethodErr>;
    fn uninhibit_sleep(&self) -> Result<(), tree::MethodErr>;
    fn inhibit_power_off(&self) -> Result<(), tree::MethodErr>;
    fn uninhibit_power_off(&self) -> Result<(), tree::MethodErr>;
    fn auto_sleep(&self) -> Result<i32, tree::MethodErr>;
    fn setauto_sleep(&self, value: i32) -> Result<(), tree::MethodErr>;
    fn sleep_inhibited(&self) -> Result<bool, tree::MethodErr>;
    fn power_off_inhibited(&self) -> Result<bool, tree::MethodErr>;
}

#[derive(Debug)]
pub struct CodesEeemsOxide1SystemLeftAction {
}

impl arg::AppendAll for CodesEeemsOxide1SystemLeftAction {
    fn append(&self, _: &mut arg::IterAppend) {
    }
}

impl arg::ReadAll for CodesEeemsOxide1SystemLeftAction {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(CodesEeemsOxide1SystemLeftAction {
        })
    }
}

impl dbus::message::SignalArgs for CodesEeemsOxide1SystemLeftAction {
    const NAME: &'static str = "leftAction";
    const INTERFACE: &'static str = "codes.eeems.oxide1.System";
}

#[derive(Debug)]
pub struct CodesEeemsOxide1SystemHomeAction {
}

impl arg::AppendAll for CodesEeemsOxide1SystemHomeAction {
    fn append(&self, _: &mut arg::IterAppend) {
    }
}

impl arg::ReadAll for CodesEeemsOxide1SystemHomeAction {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(CodesEeemsOxide1SystemHomeAction {
        })
    }
}

impl dbus::message::SignalArgs for CodesEeemsOxide1SystemHomeAction {
    const NAME: &'static str = "homeAction";
    const INTERFACE: &'static str = "codes.eeems.oxide1.System";
}

#[derive(Debug)]
pub struct CodesEeemsOxide1SystemRightAction {
}

impl arg::AppendAll for CodesEeemsOxide1SystemRightAction {
    fn append(&self, _: &mut arg::IterAppend) {
    }
}

impl arg::ReadAll for CodesEeemsOxide1SystemRightAction {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(CodesEeemsOxide1SystemRightAction {
        })
    }
}

impl dbus::message::SignalArgs for CodesEeemsOxide1SystemRightAction {
    const NAME: &'static str = "rightAction";
    const INTERFACE: &'static str = "codes.eeems.oxide1.System";
}

#[derive(Debug)]
pub struct CodesEeemsOxide1SystemPowerAction {
}

impl arg::AppendAll for CodesEeemsOxide1SystemPowerAction {
    fn append(&self, _: &mut arg::IterAppend) {
    }
}

impl arg::ReadAll for CodesEeemsOxide1SystemPowerAction {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(CodesEeemsOxide1SystemPowerAction {
        })
    }
}

impl dbus::message::SignalArgs for CodesEeemsOxide1SystemPowerAction {
    const NAME: &'static str = "powerAction";
    const INTERFACE: &'static str = "codes.eeems.oxide1.System";
}

#[derive(Debug)]
pub struct CodesEeemsOxide1SystemSleepInhibitedChanged {
    pub arg0: bool,
}

impl arg::AppendAll for CodesEeemsOxide1SystemSleepInhibitedChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.arg0, i);
    }
}

impl arg::ReadAll for CodesEeemsOxide1SystemSleepInhibitedChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(CodesEeemsOxide1SystemSleepInhibitedChanged {
            arg0: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for CodesEeemsOxide1SystemSleepInhibitedChanged {
    const NAME: &'static str = "sleepInhibitedChanged";
    const INTERFACE: &'static str = "codes.eeems.oxide1.System";
}

#[derive(Debug)]
pub struct CodesEeemsOxide1SystemPowerOffInhibitedChanged {
    pub arg0: bool,
}

impl arg::AppendAll for CodesEeemsOxide1SystemPowerOffInhibitedChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.arg0, i);
    }
}

impl arg::ReadAll for CodesEeemsOxide1SystemPowerOffInhibitedChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(CodesEeemsOxide1SystemPowerOffInhibitedChanged {
            arg0: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for CodesEeemsOxide1SystemPowerOffInhibitedChanged {
    const NAME: &'static str = "powerOffInhibitedChanged";
    const INTERFACE: &'static str = "codes.eeems.oxide1.System";
}

#[derive(Debug)]
pub struct CodesEeemsOxide1SystemAutoSleepChanged {
    pub arg0: i32,
}

impl arg::AppendAll for CodesEeemsOxide1SystemAutoSleepChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.arg0, i);
    }
}

impl arg::ReadAll for CodesEeemsOxide1SystemAutoSleepChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(CodesEeemsOxide1SystemAutoSleepChanged {
            arg0: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for CodesEeemsOxide1SystemAutoSleepChanged {
    const NAME: &'static str = "autoSleepChanged";
    const INTERFACE: &'static str = "codes.eeems.oxide1.System";
}

#[derive(Debug)]
pub struct CodesEeemsOxide1SystemDeviceSuspending {
}

impl arg::AppendAll for CodesEeemsOxide1SystemDeviceSuspending {
    fn append(&self, _: &mut arg::IterAppend) {
    }
}

impl arg::ReadAll for CodesEeemsOxide1SystemDeviceSuspending {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(CodesEeemsOxide1SystemDeviceSuspending {
        })
    }
}

impl dbus::message::SignalArgs for CodesEeemsOxide1SystemDeviceSuspending {
    const NAME: &'static str = "deviceSuspending";
    const INTERFACE: &'static str = "codes.eeems.oxide1.System";
}

#[derive(Debug)]
pub struct CodesEeemsOxide1SystemDeviceResuming {
}

impl arg::AppendAll for CodesEeemsOxide1SystemDeviceResuming {
    fn append(&self, _: &mut arg::IterAppend) {
    }
}

impl arg::ReadAll for CodesEeemsOxide1SystemDeviceResuming {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(CodesEeemsOxide1SystemDeviceResuming {
        })
    }
}

impl dbus::message::SignalArgs for CodesEeemsOxide1SystemDeviceResuming {
    const NAME: &'static str = "deviceResuming";
    const INTERFACE: &'static str = "codes.eeems.oxide1.System";
}

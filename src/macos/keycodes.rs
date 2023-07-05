// https://stackoverflow.
// com/questions/3202629/where-can-i-find-a-list-of-mac-virtual-key-codes

/* keycodes for keys that are independent of keyboard layout */

#![allow(non_upper_case_globals)]
#![allow(dead_code)]

pub const kVK_Return: u16 = 0x24;
pub const kVK_Tab: u16 = 0x30;
pub const kVK_Space: u16 = 0x31;
pub const kVK_Delete: u16 = 0x33;
pub const kVK_Escape: u16 = 0x35;
pub const kVK_Command: u16 = 0x37;
pub const kVK_Shift: u16 = 0x38;
pub const kVK_CapsLock: u16 = 0x39;
pub const kVK_Option: u16 = 0x3A;
pub const kVK_Control: u16 = 0x3B;
pub const kVK_RightShift: u16 = 0x3C;
pub const kVK_RightOption: u16 = 0x3D;
pub const kVK_RightControl: u16 = 0x3E;
pub const kVK_Function: u16 = 0x3F;
pub const kVK_F17: u16 = 0x40;
pub const kVK_VolumeUp: u16 = 0x48;
pub const kVK_VolumeDown: u16 = 0x49;
pub const kVK_Mute: u16 = 0x4A;
pub const kVK_F18: u16 = 0x4F;
pub const kVK_F19: u16 = 0x50;
pub const kVK_F20: u16 = 0x5A;
pub const kVK_F5: u16 = 0x60;
pub const kVK_F6: u16 = 0x61;
pub const kVK_F7: u16 = 0x62;
pub const kVK_F3: u16 = 0x63;
pub const kVK_F8: u16 = 0x64;
pub const kVK_F9: u16 = 0x65;
pub const kVK_F11: u16 = 0x67;
pub const kVK_F13: u16 = 0x69;
pub const kVK_F16: u16 = 0x6A;
pub const kVK_F14: u16 = 0x6B;
pub const kVK_F10: u16 = 0x6D;
pub const kVK_F12: u16 = 0x6F;
pub const kVK_F15: u16 = 0x71;
pub const kVK_Help: u16 = 0x72;
pub const kVK_Home: u16 = 0x73;
pub const kVK_PageUp: u16 = 0x74;
pub const kVK_ForwardDelete: u16 = 0x75;
pub const kVK_F4: u16 = 0x76;
pub const kVK_End: u16 = 0x77;
pub const kVK_F2: u16 = 0x78;
pub const kVK_PageDown: u16 = 0x79;
pub const kVK_F1: u16 = 0x7A;
pub const kVK_LeftArrow: u16 = 0x7B;
pub const kVK_RightArrow: u16 = 0x7C;
pub const kVK_DownArrow: u16 = 0x7D;
pub const kVK_UpArrow: u16 = 0x7E;

// US-ANSI Keyboard Layout
// From: https://github.com/socsieng/sendkeys/blob/main/Sources/SendKeysLib/KeyCodes.swift
pub mod us_ansi {
    pub const a: u16 = 0x00;
    pub const b: u16 = 0x0B;
    pub const c: u16 = 0x08;
    pub const d: u16 = 0x02;
    pub const e: u16 = 0x0E;
    pub const f: u16 = 0x03;
    pub const g: u16 = 0x05;
    pub const h: u16 = 0x04;
    pub const i: u16 = 0x22;
    pub const j: u16 = 0x26;
    pub const k: u16 = 0x28;
    pub const l: u16 = 0x25;
    pub const m: u16 = 0x2E;
    pub const n: u16 = 0x2D;
    pub const o: u16 = 0x1F;
    pub const p: u16 = 0x23;
    pub const q: u16 = 0x0C;
    pub const r: u16 = 0x0F;
    pub const s: u16 = 0x01;
    pub const t: u16 = 0x11;
    pub const u: u16 = 0x20;
    pub const v: u16 = 0x09;
    pub const w: u16 = 0x0D;
    pub const x: u16 = 0x07;
    pub const y: u16 = 0x10;
    pub const z: u16 = 0x06;

    pub const zero: u16 = 0x1D;
    pub const one: u16 = 0x12;
    pub const two: u16 = 0x13;
    pub const three: u16 = 0x14;
    pub const four: u16 = 0x15;
    pub const five: u16 = 0x17;
    pub const six: u16 = 0x16;
    pub const seven: u16 = 0x1A;
    pub const eight: u16 = 0x1C;
    pub const nine: u16 = 0x19;

    pub const equals: u16 = 0x18;
    pub const minus: u16 = 0x1B;
    pub const semicolon: u16 = 0x29;
    pub const apostrophe: u16 = 0x27;
    pub const comma: u16 = 0x2B;
    pub const period: u16 = 0x2F;
    pub const forwardSlash: u16 = 0x2C;
    pub const backslash: u16 = 0x2A;
    pub const grave: u16 = 0x32;
    pub const leftBracket: u16 = 0x21;
    pub const rightBracket: u16 = 0x1E;
}

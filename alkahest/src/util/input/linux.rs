/// Mouse buttons.
#[cfg(target_os = "linux")]
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum MouseButton {
    #[doc(hidden)]
    MouseButtonLeft = glfw::MouseButton::Button1 as u32,
    #[doc(hidden)]
    MouseButtonRight = glfw::MouseButton::Button2 as u32,
    #[doc(hidden)]
    MouseButtonMiddle = glfw::MouseButton::Button3 as u32,
    #[doc(hidden)]
    MouseButton4 = glfw::MouseButton::Button4 as u32,
    #[doc(hidden)]
    MouseButton5 = glfw::MouseButton::Button5 as u32,
    #[doc(hidden)]
    MouseButton6 = glfw::MouseButton::Button6 as u32,
    #[doc(hidden)]
    MouseButton7 = glfw::MouseButton::Button7 as u32,
    #[doc(hidden)]
    MouseButton8 = glfw::MouseButton::Button8 as u32,
}

/// Input keys.
#[cfg(target_os = "linux")]
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Key {
    #[doc(hidden)]
    Space = glfw::Key::Space as u32,
    #[doc(hidden)]
    Apostrophe = glfw::Key::Apostrophe as u32,
    #[doc(hidden)]
    Comma = glfw::Key::Comma as u32,
    #[doc(hidden)]
    Minus = glfw::Key::Minus as u32,
    #[doc(hidden)]
    Period = glfw::Key::Period as u32,
    #[doc(hidden)]
    Slash = glfw::Key::Slash as u32,
    #[doc(hidden)]
    Num0 = glfw::Key::Num0 as u32,
    #[doc(hidden)]
    Num1 = glfw::Key::Num1 as u32,
    #[doc(hidden)]
    Num2 = glfw::Key::Num2 as u32,
    #[doc(hidden)]
    Num3 = glfw::Key::Num3 as u32,
    #[doc(hidden)]
    Num4 = glfw::Key::Num4 as u32,
    #[doc(hidden)]
    Num5 = glfw::Key::Num5 as u32,
    #[doc(hidden)]
    Num6 = glfw::Key::Num6 as u32,
    #[doc(hidden)]
    Num7 = glfw::Key::Num7 as u32,
    #[doc(hidden)]
    Num8 = glfw::Key::Num8 as u32,
    #[doc(hidden)]
    Num9 = glfw::Key::Num9 as u32,
    #[doc(hidden)]
    Semicolon = glfw::Key::Semicolon as u32,
    #[doc(hidden)]
    Equal = glfw::Key::Equal as u32,
    #[doc(hidden)]
    A = glfw::Key::A as u32,
    #[doc(hidden)]
    B = glfw::Key::B as u32,
    #[doc(hidden)]
    C = glfw::Key::C as u32,
    #[doc(hidden)]
    D = glfw::Key::D as u32,
    #[doc(hidden)]
    E = glfw::Key::E as u32,
    #[doc(hidden)]
    F = glfw::Key::F as u32,
    #[doc(hidden)]
    G = glfw::Key::G as u32,
    #[doc(hidden)]
    H = glfw::Key::H as u32,
    #[doc(hidden)]
    I = glfw::Key::I as u32,
    #[doc(hidden)]
    J = glfw::Key::J as u32,
    #[doc(hidden)]
    K = glfw::Key::K as u32,
    #[doc(hidden)]
    L = glfw::Key::L as u32,
    #[doc(hidden)]
    M = glfw::Key::M as u32,
    #[doc(hidden)]
    N = glfw::Key::N as u32,
    #[doc(hidden)]
    O = glfw::Key::O as u32,
    #[doc(hidden)]
    P = glfw::Key::P as u32,
    #[doc(hidden)]
    Q = glfw::Key::Q as u32,
    #[doc(hidden)]
    R = glfw::Key::R as u32,
    #[doc(hidden)]
    S = glfw::Key::S as u32,
    #[doc(hidden)]
    T = glfw::Key::T as u32,
    #[doc(hidden)]
    U = glfw::Key::U as u32,
    #[doc(hidden)]
    V = glfw::Key::V as u32,
    #[doc(hidden)]
    W = glfw::Key::W as u32,
    #[doc(hidden)]
    X = glfw::Key::X as u32,
    #[doc(hidden)]
    Y = glfw::Key::Y as u32,
    #[doc(hidden)]
    Z = glfw::Key::Z as u32,
    #[doc(hidden)]
    LeftBracket = glfw::Key::LeftBracket as u32,
    #[doc(hidden)]
    Backslash = glfw::Key::Backslash as u32,
    #[doc(hidden)]
    RightBracket = glfw::Key::RightBracket as u32,
    #[doc(hidden)]
    GraveAccent = glfw::Key::GraveAccent as u32,
    #[doc(hidden)]
    World1 = glfw::Key::World1 as u32,
    #[doc(hidden)]
    World2 = glfw::Key::World2 as u32,
    #[doc(hidden)]
    Escape = glfw::Key::Escape as u32,
    #[doc(hidden)]
    Enter = glfw::Key::Enter as u32,
    #[doc(hidden)]
    Tab = glfw::Key::Tab as u32,
    #[doc(hidden)]
    Backspace = glfw::Key::Backspace as u32,
    #[doc(hidden)]
    Insert = glfw::Key::Insert as u32,
    #[doc(hidden)]
    Delete = glfw::Key::Delete as u32,
    #[doc(hidden)]
    Right = glfw::Key::Right as u32,
    #[doc(hidden)]
    Left = glfw::Key::Left as u32,
    #[doc(hidden)]
    Down = glfw::Key::Down as u32,
    #[doc(hidden)]
    Up = glfw::Key::Up as u32,
    #[doc(hidden)]
    PageUp = glfw::Key::PageUp as u32,
    #[doc(hidden)]
    PageDown = glfw::Key::PageDown as u32,
    #[doc(hidden)]
    Home = glfw::Key::Home as u32,
    #[doc(hidden)]
    End = glfw::Key::End as u32,
    #[doc(hidden)]
    CapsLock = glfw::Key::CapsLock as u32,
    #[doc(hidden)]
    ScrollLock = glfw::Key::ScrollLock as u32,
    #[doc(hidden)]
    NumLock = glfw::Key::NumLock as u32,
    #[doc(hidden)]
    PrintScreen = glfw::Key::PrintScreen as u32,
    #[doc(hidden)]
    Pause = glfw::Key::Pause as u32,
    #[doc(hidden)]
    F1 = glfw::Key::F1 as u32,
    #[doc(hidden)]
    F2 = glfw::Key::F2 as u32,
    #[doc(hidden)]
    F3 = glfw::Key::F3 as u32,
    #[doc(hidden)]
    F4 = glfw::Key::F4 as u32,
    #[doc(hidden)]
    F5 = glfw::Key::F5 as u32,
    #[doc(hidden)]
    F6 = glfw::Key::F6 as u32,
    #[doc(hidden)]
    F7 = glfw::Key::F7 as u32,
    #[doc(hidden)]
    F8 = glfw::Key::F8 as u32,
    #[doc(hidden)]
    F9 = glfw::Key::F9 as u32,
    #[doc(hidden)]
    F10 = glfw::Key::F10 as u32,
    #[doc(hidden)]
    F11 = glfw::Key::F11 as u32,
    #[doc(hidden)]
    F12 = glfw::Key::F12 as u32,
    #[doc(hidden)]
    F13 = glfw::Key::F13 as u32,
    #[doc(hidden)]
    F14 = glfw::Key::F14 as u32,
    #[doc(hidden)]
    F15 = glfw::Key::F15 as u32,
    #[doc(hidden)]
    F16 = glfw::Key::F16 as u32,
    #[doc(hidden)]
    F17 = glfw::Key::F17 as u32,
    #[doc(hidden)]
    F18 = glfw::Key::F18 as u32,
    #[doc(hidden)]
    F19 = glfw::Key::F19 as u32,
    #[doc(hidden)]
    F20 = glfw::Key::F20 as u32,
    #[doc(hidden)]
    F21 = glfw::Key::F21 as u32,
    #[doc(hidden)]
    F22 = glfw::Key::F22 as u32,
    #[doc(hidden)]
    F23 = glfw::Key::F23 as u32,
    #[doc(hidden)]
    F24 = glfw::Key::F24 as u32,
    #[doc(hidden)]
    F25 = glfw::Key::F25 as u32,
    #[doc(hidden)]
    Kp0 = glfw::Key::Kp0 as u32,
    #[doc(hidden)]
    Kp1 = glfw::Key::Kp1 as u32,
    #[doc(hidden)]
    Kp2 = glfw::Key::Kp2 as u32,
    #[doc(hidden)]
    Kp3 = glfw::Key::Kp3 as u32,
    #[doc(hidden)]
    Kp4 = glfw::Key::Kp4 as u32,
    #[doc(hidden)]
    Kp5 = glfw::Key::Kp5 as u32,
    #[doc(hidden)]
    Kp6 = glfw::Key::Kp6 as u32,
    #[doc(hidden)]
    Kp7 = glfw::Key::Kp7 as u32,
    #[doc(hidden)]
    Kp8 = glfw::Key::Kp8 as u32,
    #[doc(hidden)]
    Kp9 = glfw::Key::Kp9 as u32,
    #[doc(hidden)]
    KpDecimal = glfw::Key::KpDecimal as u32,
    #[doc(hidden)]
    KpDivide = glfw::Key::KpDivide as u32,
    #[doc(hidden)]
    KpMultiply = glfw::Key::KpMultiply as u32,
    #[doc(hidden)]
    KpSubtract = glfw::Key::KpSubtract as u32,
    #[doc(hidden)]
    KpAdd = glfw::Key::KpAdd as u32,
    #[doc(hidden)]
    KpEnter = glfw::Key::KpEnter as u32,
    #[doc(hidden)]
    KpEqual = glfw::Key::KpEqual as u32,
    #[doc(hidden)]
    LeftShift = glfw::Key::LeftShift as u32,
    #[doc(hidden)]
    LeftControl = glfw::Key::LeftControl as u32,
    #[doc(hidden)]
    LeftAlt = glfw::Key::LeftAlt as u32,
    #[doc(hidden)]
    LeftSuper = glfw::Key::LeftSuper as u32,
    #[doc(hidden)]
    RightShift = glfw::Key::RightShift as u32,
    #[doc(hidden)]
    RightControl = glfw::Key::RightControl as u32,
    #[doc(hidden)]
    RightAlt = glfw::Key::RightAlt as u32,
    #[doc(hidden)]
    RightSuper = glfw::Key::RightSuper as u32,
    #[doc(hidden)]
    Menu = glfw::Key::Menu as u32,
    #[doc(hidden)]
    Unknown = glfw::Key::Unknown as u32,
}

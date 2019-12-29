use cpp::*;

cpp! {{
    #include <Nazara/Core/Core.hpp>
}}

pub unsafe fn initialize() -> bool {
    cpp!([] -> bool as "bool" {
        return Nz::Core::Initialize();
    })
}

pub unsafe fn is_initialized() -> bool {
    cpp!([] -> bool as "bool" {
        return Nz::Core::IsInitialized();
    })
}

pub unsafe fn uninitialize() {
    cpp!([] {
        Nz::Core::Uninitialize();
    });
}

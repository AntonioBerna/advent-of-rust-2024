use std::marker::PhantomData;

// Define the state structs
pub struct Empty;
pub struct Ready;
pub struct Flying;

// Define the generic Sleigh struct
pub struct Sleigh<State> {
    _state: PhantomData<State>,
}

// Implement methods for the Empty state
impl Sleigh<Empty> {
    // Create a new Sleigh in the Empty state
    pub fn new() -> Self {
        Self {
            _state: PhantomData,
        }
    }

    // Transition to the Ready state
    pub fn load(self) -> Sleigh<Ready> {
        Sleigh {
            _state: PhantomData,
        }
    }
}

// Implement methods for the Ready state
impl Sleigh<Ready> {
    // Transition to the Flying state
    pub fn take_off(self) -> Sleigh<Flying> {
        Sleigh {
            _state: PhantomData,
        }
    }

    // Transition back to the Empty state
    pub fn unload(self) -> Sleigh<Empty> {
        Sleigh {
            _state: PhantomData,
        }
    }
}

// Implement methods for the Flying state
impl Sleigh<Flying> {
    // Transition back to the Ready state
    pub fn land(self) -> Sleigh<Ready> {
        Sleigh {
            _state: PhantomData,
        }
    }
}

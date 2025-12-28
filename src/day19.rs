// 1. We have 3 states:
// - Empty
// - Ready
// - Flying

use std::marker::PhantomData;

pub struct Empty;
pub struct Ready;
pub struct Flying;

// 2. Finish the Seligh struct definition
pub struct Sleigh<State> {
    _state: PhantomData<State>
}

// 3. Write the Sleigh Implementations for all states
impl Sleigh<Empty> {
    pub fn new() -> Sleigh<Empty> {
        Self {
            _state: PhantomData
        }
    }

    pub fn load(self) -> Sleigh<Ready> {
        Sleigh { _state: PhantomData }
    }
}

impl Sleigh<Ready> {
    pub fn take_off(self) -> Sleigh<Flying> {
        Sleigh { _state: PhantomData }
    }

    pub fn unload(self) -> Sleigh<Empty> {
        Sleigh { _state: PhantomData }
    }
}

impl Sleigh<Flying> {

    pub fn land(self) -> Sleigh<Ready> {
        Sleigh { _state: PhantomData}
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sleigh_flow() {
        let sleigh = Sleigh::new();
        // Initial state is Empty
        let sleigh = sleigh.load();
        // Now it is Ready
        let sleigh = sleigh.take_off();
        // Now it is Flying
        let sleigh = sleigh.land();
        // Now it is Ready again
        let _ = sleigh.unload();
        // Back to Empty
    }

    #[test]
    fn test_sleigh_load_unload() {
        let sleigh = Sleigh::new();
        let sleigh = sleigh.load();
        let _ = sleigh.unload();
    }
}

use std::collections::HashMap;

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(u64);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<T> {
    input_cells: HashMap<InputCellId, T>,
    compute_cells: HashMap<ComputeCellId, (Vec<CellId>, Box<dyn Fn(&[T]) -> T>)>,
    callbacks: HashMap<ComputeCellId, HashMap<CallbackId, Box<dyn FnMut(T)>>>,
    id_counter: u64,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<T: Copy + PartialEq> Reactor<T> {
    pub fn new() -> Self {
        Reactor {
            input_cells: HashMap::new(),
            compute_cells: HashMap::new(),
            callbacks: HashMap::new(),
            id_counter: 0,
        }
    }

    fn next_id(&mut self) -> u64 {
        self.id_counter += 1;
        self.id_counter
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, _initial: T) -> InputCellId {
        let id = InputCellId(self.next_id());
        self.input_cells.insert(id, _initial);
        id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(
        &mut self,
        _dependencies: &[CellId],
        _compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        let id = ComputeCellId(self.next_id());
        for dependency in _dependencies.iter() {
            match dependency {
                CellId::Input(id) => {
                    if !self.input_cells.contains_key(id) {
                        return Err(CellId::Input(*id));
                    }
                }
                CellId::Compute(id) => {
                    if !self.compute_cells.contains_key(id) {
                        return Err(CellId::Compute(*id));
                    }
                }
            }
        }
        self.compute_cells
            .insert(id, (_dependencies.to_vec(), Box::new(_compute_func)));
        Ok(id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(id) => self.input_cells.get(&id).copied(),
            CellId::Compute(id) => {
                let (dependencies, compute_func) = self.compute_cells.get(&id)?;
                let mut values = Vec::new();
                for dependency in dependencies.iter() {
                    match dependency {
                        CellId::Input(id) => values.push(self.input_cells.get(&id).copied()?),
                        CellId::Compute(id) => {
                            values.push(self.value(CellId::Compute(*id))?);
                        }
                    }
                }
                Some(compute_func(&values))
            }
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, _id: InputCellId, _new_value: T) -> bool {
        if !self.input_cells.contains_key(&_id) {
            return false;
        }
        let current_value = self.input_cells.get(&_id).unwrap();
        for (compute_id, (dependencies, compute_func)) in self.compute_cells.iter() {
            if dependencies.contains(&CellId::Input(_id)) {
                let mut values = Vec::new();
                for dependency in dependencies.iter() {
                    match dependency {
                        CellId::Input(id) => {
                            values.push(self.input_cells.get(&id).copied().unwrap())
                        }
                        CellId::Compute(id) => {
                            values.push(self.value(CellId::Compute(*id)).unwrap());
                        }
                    }
                }
                let new_value = compute_func(&values);
                if new_value != *current_value {
                    for callback in self.callbacks.get_mut(compute_id).unwrap().values_mut() {
                        callback(new_value);
                    }
                }
            }
        }
        true
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'static>(
        &mut self,
        _id: ComputeCellId,
        _callback: F,
    ) -> Option<CallbackId> {
        if self.compute_cells.contains_key(&_id) {
            let id = CallbackId(self.next_id());
            self.callbacks
                .entry(_id)
                .or_insert_with(HashMap::new)
                .insert(id, Box::new(_callback));
            Some(id)
        } else {
            None
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if !self.compute_cells.contains_key(&cell) {
            return Err(RemoveCallbackError::NonexistentCell);
        }
        if !self.callbacks.contains_key(&cell) {
            return Err(RemoveCallbackError::NonexistentCallback);
        }
        self.callbacks.get_mut(&cell).unwrap().remove(&callback);
        Ok(())
    }
}

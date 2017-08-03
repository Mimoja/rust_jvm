
pub struct Heap(Vec<Variable>);

#[derive(Clone, Debug)]
pub enum Variable{
    char(u8),
    short(u16),
    int(u32),
    float(f32),
    pointer(),
    object(),
}

impl Heap {
    pub fn get(&mut self, index: u8) -> &mut Variable {
        if let Some(l) = self.0.get_mut(index as usize) {
            return l;
        } else {
            panic!("local val {} does not exist", index);
        }
    }

}
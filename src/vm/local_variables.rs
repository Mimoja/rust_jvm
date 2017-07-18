pub struct LocalVariables(Vec<u32>);

impl LocalVariables {
    pub fn get(&mut self, index: u8) -> &mut u32 {
        if let Some(l) = self.0.get_mut(index as usize) {
            return l;
        } else {
            panic!("local val {} does not exist", index);
        }
    }
}
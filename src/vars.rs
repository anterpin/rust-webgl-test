use super::shader_program;

struct Store<T> {
    list: Vec<T>,
}
impl<T> Store<T> {
    fn new(obj: T) -> Store<T> {
        let mut list = Vec::new();
        list.push(obj);
        Store { list: list }
    }
    fn add(&mut self, obj: T) -> i32 {
        self.list.push(obj);
        (self.list.len() - 1) as i32
    }
    fn get(&self, id: usize) -> Option<&T> {
        if id >= self.list.len() {
            None
        } else {
            Some(&self.list[id])
        }
    }
}
type ShaderPrograms = Store<shader_program::ShaderProgram>;
static mut PROGRAMS: Option<Box<ShaderPrograms>> = None;

pub fn add_program(program: shader_program::ShaderProgram) -> i32 {
    unsafe {
        if PROGRAMS.is_none() {
            PROGRAMS = Some(Box::new(Store::new(program)));
            return 0;
        } else {
            PROGRAMS.as_deref_mut().unwrap().add(program)
        }
    }
}

pub fn get_program(id: usize) -> Option<&'static shader_program::ShaderProgram> {
    unsafe {
        if PROGRAMS.is_none() {
            None
        } else {
            PROGRAMS.as_deref().unwrap().get(id)
        }
    }
}

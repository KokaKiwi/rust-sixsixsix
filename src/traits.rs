pub trait Something {
    fn get_name(&self) -> &str;
}

pub trait Entity : Something {
    fn get_function(&self) -> &str;

    fn summon(&self) {
        println!("I summon you {0} ! All hail {0}, {1} !", self.get_name(), self.get_function());
    }
}

pub trait Expendable : Something + Sized {
    fn sacrifice_to<T: Entity>(self, entity: &T) {
        println!("Let's sacrifice {} to {}", self.get_name(), entity.get_name());
    }
}

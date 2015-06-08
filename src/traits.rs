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

impl<T: Something> Something for Vec<T> {
    fn get_name(&self) -> &str {
        match self.len() {
            0 => "nothing",
            _ => self[0].get_name(),
        }
    }
}

impl<T: Expendable> Expendable for Vec<T> {
    fn sacrifice_to<E: Entity>(self, entity: &E) {
        for thing in self {
            thing.sacrifice_to(entity);
        }
    }
}

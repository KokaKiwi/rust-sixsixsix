#![macro_use]

macro_rules! create_something(
    ($name:ident) => (
        pub struct $name {
            name: String
        }

        impl $name {
            pub fn new(name: &str) -> $name {
                $name {
                    name: name.to_owned(),
                }
            }
        }

        impl ::Something for $name {
            fn get_name(&self) -> &str {
                &self.name
            }
        }

        impl ::Expendable for $name {}
    );
);

macro_rules! create_entity(
    ($name:ident) => (
        pub struct $name {
            name: String,
            function: String
        }

        impl $name {
            pub fn new(name: &str, function: &str) -> $name {
                $name {
                    name: name.to_owned(),
                    function: function.to_owned(),
                }
            }
        }

        impl ::Something for $name {
            fn get_name(&self) -> &str {
                &self.name
            }
        }

        impl ::Entity for $name {
            fn get_function(&self) -> &str {
                &self.function
            }
        }
    );
);
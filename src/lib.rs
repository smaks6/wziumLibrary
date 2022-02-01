#[cfg(test)]
mod tests {
    use crate::wzium_lib::wzium;

    #[test]
    fn it_works() {
        println!("{}", wzium(true));
    }
}

pub mod wzium_lib{
    pub fn wzium(print_certificate: bool)-> String{
        if print_certificate{
            println!("===============================");
            println!("This is wzium library!");
            println!("===============================");
        }
        String::from("WZIUM")
    }
}

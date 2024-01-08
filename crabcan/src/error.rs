use std::fmt;
use std::process::exit;

//// trait Display, allows Errcode enum to be displayed by:
//   println!("{}", error);
//  in this case, it calls the function "fmt", which we define the behaviour below
#[allow(unreachable_patterns)]
impl fmt::Display for Err {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        //Defining what behaviour for each enum would be given
        match &self{
            Err::ArgumentInvalid(element) => write!(f , "ArgumentInvalid: {}" , element),
            
            _ => write!(f , "{:?}" , self)
        }
    }
}

pub fn exit_returncode(res : Result<(), Err>) {
    match res {
        Ok(_) => {
            log::debug!("Exiting without any err");
            exit(0);
        },
        Err(e) => {
            let return_code = e.get_return_code();
            log::error!("Error on exit:\n\t{}\n\tReturning {} ", e , return_code);
            exit(return_code);
        }
    }
}

#[derive(Debug)]
pub enum Err{
    ArgumentInvalid(&'static str),
    ContainerError(u8),
    NotSupported(u8)
}
impl Err{
    //translate an error code::X into a number to return {the UNIX way}
    pub fn get_return_code(&self) -> i32 {
        1 //we are using this number to signify every != 0 error 
    }
}
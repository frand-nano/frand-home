use std::any::Any;

pub trait StateMessage {
    fn error(err: String) -> Self;
    
    fn new(
        ids: &[usize], 
        index: usize, 
        #[allow(unused_variables)] value: Box<dyn Any>,    
    ) -> Self 
    where Self: Sized
    {
        match Self::try_new(ids, index, value) {
            Ok(result) => result,
            Err(err) => {
                let err = format!("❗ StateMessage::new 
                    ids:{:#?}, 
                    index:{index}, 
                    err:{:#?}, 
                ", ids, err);
                
                log::error!("{err}");
                Self::error(err)
            },
        }
    }

    fn try_new(
        ids: &[usize], 
        index: usize, 
        #[allow(unused_variables)] value: Box<dyn Any>,
    ) -> Result<Self, Box<dyn Any>> where Self: Sized;    
}
use crate::{Task,Tasks};

pub struct taskrow {
    taskscolumns: Vec<Option<Vec<u8>>>,
}
#[derive(Debug)]
pub enum DbError {
    Null,
    InvalidValue,
    InvalidUtf8,
    Io(std::io::Error),
}
impl taskrow {
    pub fn geti32(&self, index : usize) -> Result<i32 , DbError>{
            match self.taskscolumns[index].as_ref() {
                Some(value) => {
                    let bytes = value;
                    let bytes : [u8 ; 4 ] = bytes.as_slice().try_into().map_err(|_| DbError::InvalidValue)?;
                    return Ok(i32::from_be_bytes(bytes))
                }
                None => {return Err(DbError::InvalidValue)}
            
            }
            
    }
    pub fn getstring(&self , index : usize)->Result<String , DbError>{
        match self.taskscolumns[index].as_ref() {
            Some(value) => {
                return Ok(String::from_utf8(value.clone()).map_err(|_| DbError::InvalidValue)?)
            },
            None => {
                return Err(DbError::InvalidValue)
            }
        }
    }
    pub fn getbool(&self , index : usize)->Result<bool , DbError>{
         match self.taskscolumns[index].as_ref() {
            Some(value) => {
                match value.as_slice() {
                    [0] => Ok(false),
                    [1] => Ok(true) ,
                    _=> Err(DbError::InvalidValue)
                }
            },
            None => {
                return Err(DbError::InvalidValue)
            }
        }
    }

    pub fn extract(&self)->Result<Task , DbError>{
          Ok(Task {
            name : self.getstring(0)?, 
            id: self.geti32(1)? ,
            done :self.getbool(2)?  
        })
    }

}
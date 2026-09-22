result!(
    Ok;
    "AHuman Ok";
    usize;
    [
        [1; AMPLE_DEFAULT_OK; Default; usize; "ZE"; "Entry to ze"],
    ];
    Error;
    "Human error";
    usize;
    [
        [1; AMPLE_DEFAULT_ERROR; Error; usize; "ZE"; "Entry to ze"],
    ]
);

impl Ok {
    pub fn from_no(no: usize) -> Self {
        Ok::Default(no)
    }
}

impl Error {
    pub fn from_no(no: usize) -> Self {
        Error::Error(no)
    }
}

pub type Result = core::result::Result<Ok, Error>;

pub fn handle_result(result: usize) -> Result {
    if (result as isize) < 0 {
        Err(Error::from_no(result))
    } else {
        Ok(Ok::from_no(result))
    }
}

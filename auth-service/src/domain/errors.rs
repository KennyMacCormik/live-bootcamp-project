pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Debug)]
pub enum EmailError {
    InvalidEmail,
}

#[derive(Debug)]
pub enum PasswordError {
    BadLength,
    WeakPassword,
}

#[derive(Debug)]
pub enum UserError {
    EmailParsingError(EmailError),
    PasswordParsingError(PasswordError),
}
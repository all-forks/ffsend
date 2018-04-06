use ffsend_api::action::delete::Error as DeleteError;
use ffsend_api::action::download::Error as DownloadError;
use ffsend_api::action::params::Error as ParamsError;
use ffsend_api::action::password::Error as PasswordError;
use ffsend_api::action::upload::Error as UploadError;
use ffsend_api::file::remote_file::FileParseError;

use action::info::Error as InfoError;

#[derive(Fail, Debug)]
pub enum Error {
    /// An error occurred while invoking an action.
    #[fail(display = "")]
    Action(#[cause] ActionError),
}

impl From<InfoError> for Error {
    fn from(err: InfoError) -> Error {
        Error::Action(ActionError::Info(err))
    }
}

impl From<ActionError> for Error {
    fn from(err: ActionError) -> Error {
        Error::Action(err)
    }
}

#[derive(Debug, Fail)]
pub enum ActionError {
    /// An error occurred while invoking the delete action.
    #[fail(display = "Failed to delete the file")]
    Delete(#[cause] DeleteError),

    /// An error occurred while invoking the download action.
    #[fail(display = "Failed to download the requested file")]
    Download(#[cause] DownloadError),

    /// An error occurred while invoking the info action.
    #[fail(display = "Failed to fetch file info")]
    Info(#[cause] InfoError),

    /// An error occurred while invoking the params action.
    #[fail(display = "Failed to change the parameters")]
    Params(#[cause] ParamsError),

    /// An error occurred while invoking the password action.
    #[fail(display = "Failed to change the password")]
    Password(#[cause] PasswordError),

    /// An error occurred while invoking the upload action.
    // TODO: bind the upload cause here
    #[fail(display = "Failed to upload the specified file")]
    Upload(#[cause] UploadError),

    /// Failed to parse a share URL, it was invalid.
    /// This error is not related to a specific action.
    #[fail(display = "Invalid share URL")]
    InvalidUrl(#[cause] FileParseError),
}

impl From<DeleteError> for ActionError {
    fn from(err: DeleteError) -> ActionError {
        ActionError::Delete(err)
    }
}

impl From<DownloadError> for ActionError {
    fn from(err: DownloadError) -> ActionError {
        ActionError::Download(err)
    }
}

impl From<InfoError> for ActionError {
    fn from(err: InfoError) -> ActionError {
        ActionError::Info(err)
    }
}

impl From<ParamsError> for ActionError {
    fn from(err: ParamsError) -> ActionError {
        ActionError::Params(err)
    }
}

impl From<PasswordError> for ActionError {
    fn from(err: PasswordError) -> ActionError {
        ActionError::Password(err)
    }
}

impl From<UploadError> for ActionError {
    fn from(err: UploadError) -> ActionError {
        ActionError::Upload(err)
    }
}

impl From<FileParseError> for ActionError {
    fn from(err: FileParseError) -> ActionError {
        ActionError::InvalidUrl(err)
    }
}
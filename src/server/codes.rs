/// This enum is used to represent the possible requests a user could make.
/// Valid requests are Add and Get.
/// 'Add' means to add a new user to the key storage structure.
/// 'Get' means to retrieve an existing user from the stucture.
enum RequestCodes {
    Add,
    Get
}

/// This enum is used to represent the possible responses the server could return to the user.
/// 'Valid' means that not errors occured.
/// 'InvalidRequestType' means that the user has made a request that is not valid (see RequestCodes enum).
/// 'CannotReadUsername' means that an error occured while trying to read name given by user.
/// 'CannotReadPublicKey' means that an error occured while trying to read public key given by user.
/// 'UsernameNotFound' occurs when the user tries to query a username. This occurs when the queried username is not stored.
/// 'UsernameAlreadyExists' occurs when the user tries to add a user. This occurs when the given username is already being stored.
/// 'CannotLockMutex' occurs when a Mutex type cannot be used (aka locked).
/// 'CannotWriteToStream' occurs when the server cannot return a response to the user.
enum ResponseCodes {
    Valid,                  // 200
    InvalidRequestType,     // 500
    CannotReadUsername,     // 501
    CannotReadPublicKey,    // 502
    UsernameNotFound,       // 503
    UsernameAlreadyExists,  // 504
    CannotLockMutex,        // 505
    CannotWriteToStream     // 506
}
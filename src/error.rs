// use opencv::Error as OpencvError;
// use std::io;
// use kafka::Error as KafkaError;

// #[derive(Debug)]
// pub enum Error {
//     KafkaError(KafkaError),
//     OpencvError(OpencvError),
//     IoError(io::Error),
//     StringError(String),
// }

// impl From<KafkaError> for Error {
//     fn from(err: KafkaError) -> Self {
//         Error::KafkaError(err)
//     }
// }

// impl From<OpencvError> for Error {
//     fn from(err: OpencvError) -> Self {
//         Error::OpencvError(err)
//     }
// }

// impl From<io::Error> for Error {
//     fn from(err: io::Error) -> Self {
//         Error::IoError(err)
//     }
// }


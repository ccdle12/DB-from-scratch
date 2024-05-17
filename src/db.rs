use crate::Result;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader, BufWriter};
use std::path::Path;

/// The client driver that interfaces with the in-memory and on disk DB.
pub struct DBDriver {
    dir: String,
}

impl DBDriver {
    pub fn new<T: ToString>(dir_folder: T) -> Self {
        DBDriver {
            dir: dir_folder.to_string(),
        }
    }

    pub fn write(&self, id: u128, input: &[u8]) -> Result<()> {
        let path_str = format!("{}/{}.json", &self.dir, &id);
        let db_file_path = Path::new(&path_str);

        // Create file if it doesn't exist.
        let db_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(db_file_path)?;

        let _ = BufWriter::new(db_file).write(input)?;

        Ok(())
    }

    pub fn read(&self, id: u128) -> Result<String> {
        let path_str = format!("{}/{}.json", &self.dir, &id);
        let db_file_path = Path::new(&path_str);

        let db_file = File::open(db_file_path)?;
        let mut reader = BufReader::new(db_file);

        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;

        Ok(buffer)
    }
}

mod test {
    use super::*;
    use crate::User;
    use std::fs::{create_dir, remove_dir_all};

    #[test]
    fn read_write_test() {
        let test_folder = "./tmp";
        create_dir(test_folder).unwrap();

        let db = DBDriver::new(test_folder);

        let user_0 = User {
            id: 0,
            name: "John".into(),
            age: 20,
        };
        let user_json = serde_json::to_string(&user_0).unwrap();

        db.write(user_0.id, &user_json.as_bytes()).unwrap();

        // Assert we can read the same value from the DB using the id.
        let result = db.read(user_0.id).unwrap();
        let read_user: User = serde_json::from_str(&result).unwrap();

        assert_eq!(read_user, user_0);

        // Write another user to the DB, this should be a new file.
        let user_1 = User {
            id: 1,
            name: "Alice".into(),
            age: 30,
        };
        let user_json_1 = serde_json::to_string(&user_1).unwrap();
        db.write(user_1.id, &user_json_1.as_bytes()).unwrap();

        let result_1 = db.read(user_1.id).unwrap();
        let read_user_1: User = serde_json::from_str(&result_1).unwrap();

        assert_eq!(read_user_1, user_1);

        remove_dir_all(test_folder).unwrap();
    }
}

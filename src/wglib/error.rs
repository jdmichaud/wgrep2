use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub enum WGErrCode {
  FileDoesNotExist,
  CannotOpenFile,
}

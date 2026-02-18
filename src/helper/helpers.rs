use git2::Status;

#[derive(Default)]
pub struct Helper;

impl Helper {
    pub fn get_txt_icon(self, st: Status) -> String {
        match st {
            Status::WT_MODIFIED => "M".to_string(),
            Status::WT_RENAMED => "R".to_string(),
            Status::WT_DELETED => "D".to_string(),
            Status::WT_TYPECHANGE => "T".to_string(),
            Status::WT_NEW => "N".to_string(),
            Status::WT_UNREADABLE => "U".to_string(),
            _ => "??".to_string(),
        }
    }
}

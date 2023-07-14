use crate::{StreamError, constant};
use std::fmt::{Display, Formatter};
use std::time::SystemTime;
use textwrap::wrap;

/// Post struct which is the heart of this project.
#[derive(Debug, Clone)]
pub struct Post {
    pub title: String,
    pub msg: String,
    pub date: SystemTime,
}

impl Post {
    /// Create a new post by providing the `title` and the body
    /// of the message in `msg`.
    pub fn new(title: String, msg: String) -> Result<Self, StreamError> {
        verify_title(&title)?;
        verify_msg(&msg)?;
        Ok(Post {
            title,
            msg,
            date: SystemTime::now(),
        })
    }

    pub fn update_msg(&mut self, new_msg: String) -> Result<(), StreamError> {
        verify_msg(&new_msg)?;
        self.msg = new_msg;
        Ok(())
    }

    pub fn update_title(&mut self, new_title: String) -> Result<(), StreamError> {
        verify_title(&new_title)?;
        self.title = new_title;
        Ok(())
    }
}

/// Some necessary checks for post title.
fn verify_title(title: &String) -> Result<(), StreamError> {
    if title.is_empty() {
        return Err(StreamError::EmptyTitle);
    }
    if title.len() > constant::MAX_TITLE_LEN {
        return Err(StreamError::InvalidTitleLength {
            max_size: MAX_TITLE_LEN,
            curr_size: title.len(),
        });
    }
    Ok(())
}

/// Some necessary checks for post message.
fn verify_msg(msg: &String) -> Result<(), StreamError> {
    // check min/max length of post
    if msg.is_empty() {
        return Err(StreamError::EmptyPost);
    }
    if msg.len() > constant::MAX_POST_LEN as usize {
        return Err(StreamError::InvalidPostLength {
            max_size: MAX_POST_LEN,
            curr_size: msg.len(),
        });
    }
    Ok(())
}

impl Display for Post {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:-<54}\n", "")?;
        write!(f, "\\ {:^50} /\n/ {:50} \\\n", self.title, "")?;
        let mut count = 0u8;
        for line in wrap(&format!("{}\n", self.msg), 50) {
            let (left_closure, right_closure) = if count % 2 == 0 {
                ("\\ ", " /")
            } else {
                ("/ ", " \\")
            };
            write!(f, "{left_closure}{: <50}{right_closure}\n", line)?;
            count += 1;
        }
        write!(f, "{:-<54}", "")
    }
}

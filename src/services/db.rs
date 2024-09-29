use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

use crate::models::user_model::User;

pub(crate) type UserDb = Arc<Mutex<HashMap<i32, User>>>;
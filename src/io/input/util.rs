pub mod contact;
pub mod employee_status;
pub mod location;
pub mod organization;
pub mod person;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct SerdeWrapper<T> { value: T }

mod enums;
mod options;
mod matches;

use crate::enums::enums;
use crate::options::options;
use crate::matches::matches;

fn main() {
    // Enums or Enumerations:
    enums();

    // Option enum : Built-in enum for Some(val) and None:
    options();

    // match : to match series of patterns and decide the actions:
    matches();

}

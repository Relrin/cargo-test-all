mod traits;

use crate::runners::traits::TestRunner;

pub fn get_runner() -> Box<dyn TestRunner> {}

/* This module should be used to get data from the system config files ~/.config/gum/
 * - profiles.yaml <- Here urls and tokens should be stores
 */

use std::collections::HashMap;


struct Profile {
    url: String,
    token: String,
}

impl Profile {
    fn new(url: String, token: String) -> Self { Self { url, token } }
}

struct Profiles {
    profiles: HashMap<String, Profile>
}

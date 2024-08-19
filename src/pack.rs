
use version;
use virtual;

pub struct Modpack {
    title: String,
    version: Version,
    modloader: Prototype<Mod>,
    
}

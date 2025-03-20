#[derive(Clone)]
pub struct Identity {
    pub uid: String,
}
impl Identity {
    pub fn authenticate(token: String) -> Option<Identity> {
        let i = Self { uid: token };
        println!("authenticating... {}", i.uid);
        return Some(i);
    }
}

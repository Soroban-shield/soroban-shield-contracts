pub trait PausableInterface {
    fn pause();
    fn unpause();
    fn is_paused() -> bool;
}

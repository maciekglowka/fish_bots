use std::sync::mpsc;

pub struct Console {
    tx: mpsc::Sender<String>,
    rx: mpsc::Receiver<String>,
}
impl Console {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self { tx, rx }
    }
    pub fn get_sender(&self) -> mpsc::Sender<String> {
        self.tx.clone()
    }
    pub fn read(&self) -> Vec<String> {
        let mut v = Vec::new();
        while let Ok(s) = self.rx.try_recv() {
            v.push(s);
        }
        v
    }
    pub fn send(&self, s: String) {
        let _ = self.tx.send(s);
    }
}

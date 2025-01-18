use tauri::{CloseRequestApi, WindowEvent};

pub enum CustomWindowEvent {
    Real(WindowEvent),
    #[allow(unused)]
    FakeCloseRequested {
        api: Box<dyn CloseRequestApiLike>,
    },
}


pub trait CloseRequestApiLike {
    fn prevent_close(&self);
}

impl CloseRequestApiLike for CloseRequestApi {
    fn prevent_close(&self) {
        self.prevent_close();
    }
}

#![allow(unused)]

use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use futures::StreamExt;

pub struct Signal<T> {
    phantom: PhantomData<T>,
}

pub struct SignalMut<T> {
    tx: Arc<Mutex<futures::channel::mpsc::Sender<T>>>,
}

impl<T> SignalMut<T> {
    pub async fn set(&mut self, value: T) {
        let mut lock = self.tx.lock().unwrap();
        let _ = lock.try_send(value);
    }
}

async fn signal_reader<T>(mut receiver: futures::channel::mpsc::Receiver<T>) {
    while let Some(value) = receiver.next().await {}
}

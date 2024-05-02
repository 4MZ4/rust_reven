use std::sync::Arc;
pub trait CallbackData {}

#[derive(Debug)]
pub struct MyCallbackData {//pub struct MyCallbackData<'a> {
    pub data: Arc<[u8]>, //data: &'a [u8],
}

impl CallbackData for MyCallbackData {} //impl<'a> CallbackData for MyCallbackData<'a> {}

pub struct MyCallback<T: CallbackData> {
    pub callback: Box<dyn Fn(&T)>,
}

pub trait MyTrait<T: CallbackData> {
    fn set_callback(&mut self, cb: MyCallback<T>);
    fn do_something(&self);
}

pub struct MyStruct<T: CallbackData> {
    pub callbacks: Vec<MyCallback<T>>,
    pub data: [u8; 3],
}

impl MyTrait<MyCallbackData> for MyStruct<MyCallbackData> {     //impl<'a> MyTrait<MyCallbackData<'a>> for MyStruct<MyCallbackData<'a>> {
    fn set_callback(&mut self, cb: MyCallback<MyCallbackData>) {    //    fn set_callback(&mut self, cb: MyCallback<MyCallbackData<'a>>) {
        self.callbacks.push(cb);
    }

    fn do_something(&self) {

        for cb in &self.callbacks {
            let cb_data = MyCallbackData {
                data: Arc::clone(&self.get_arc_data()), //data: &self.data,
            };

            (cb.callback)(&cb_data);
        }
    }
}
// 
impl<T: CallbackData> MyStruct<T> {
    fn get_arc_data(&self) -> Arc<[u8]> {
        Arc::new(self.data)
    }
}

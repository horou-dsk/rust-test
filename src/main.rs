mod player;
mod room;
mod websocket_server;
mod struct_test;
mod enum_test;
mod generic_test;
mod http_test;
mod macro_test;

fn main() {
    /*struct_test::run();
    enum_test::run();
    generic_test::run();
    macro_test::run();*/
    websocket_server::run();
}

//https://github.com/Foxicution/wgpu-template/blob/master/src/shader.wgsl base template
//TODO - Remove weird old comment system I used
mod app;        //0-Adds the app.rs module
mod graphics;   //0-Adds the graphic.rs module
mod camera;
mod dir;
mod brd_utilities;

use crate::{app::App, graphics::Graphics};          //3-Crate is a path prefix so it shows the rust_code_pw   
use winit::event_loop::{ControlFlow, EventLoop};    //1-Use winit functions
//use bytemuck::{Pod, Zeroable};

#[cfg(target_arch = "wasm32")]                      //1-Does not matter becuase this is not part of my system
fn run_app(event_loop: EventLoop<Graphics>, app: App) {
    // Sets up panics to go to the console.error in browser environments
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Error).expect("Couldn't initialize logger");

    // Runs the app async via the browsers event loop
    use winit::platform::web::EventLoopExtWebSys;
    wasm_bindgen_futures::spawn_local(async move {
        event_loop.spawn_app(app);
    });
}

#[cfg(not(target_arch = "wasm32"))]                                     //1-Checks if NOT using a webassebler called wasm32
fn run_app(event_loop: EventLoop<Graphics>, mut app: App) {             //2-Creates run_app fuction uses the event loop form graphics and a changable app struct from app
    // Allows the setting of the log level through RUST_LOG env var.    
    // It also allows wgpu logs to be seen.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error")).init();    //3-Sets up the event logger so errors create logs

    // Runs the app on the current thread.
    let _ = event_loop.run_app(&mut app);                               //3-Creats the event loop using the changeable app let _ means that it does not have a variable name but the code still runs
}

fn main() {                                                                                                     //0-Main code where scrips run
    // <T> (T -> AppEvent) extends regular platform specific events (resize, mouse, etc.).
    // This allows our app to inject custom events and handle them alongside regular ones.
    // let event_loop = EventLoop::<()>::new().unwrap();

    //0-let event_loop which is the name
    //3-EventLoop<Graphics> is the type im not sure what this type is but it works
    //2-EventLoop::<Graphics> creates the variable from the graphics struc which is derived from wgpu
    //2-with_user_event().build().unwrap() this builds the event loop I dont know what unwrap does
    let event_loop = EventLoop::<Graphics>::with_user_event().build().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.

    //1-Runs the event loop and when the event loop ends starts it again
    event_loop.set_control_flow(ControlFlow::Poll);


    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    //event_loop.set_control_flow(ControlFlow::Wait);


    //1-creating the app variable using the App type from wgpu
    //1-&event_loop the & "borrows" the event loop value
    let app = App::new(&event_loop);
    //run_app see in main.rs
    run_app(event_loop, app);
}
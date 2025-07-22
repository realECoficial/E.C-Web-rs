mod components;

use dioxus::prelude::*;
use crate::components::*;

#[allow(non_snake_case)]
const MAIN_CSS: Asset = asset!("/assets/main.css");


fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    
    }
}
/*
 *TODO usar esto en el final, aun estoy aprendiendo a linkear paginas.
            a {href: IMAGENES_HTML, 
            h1 {  "Fondos de pantalla" }
            } 
            p { "Aca guardo imagenes que encuentro yume 2kki" } 
 * */

#[derive(Routable, PartialEq, Clone)]
#[rustfmt::skip]
enum Route {
 #[layout(NavBar)]
    #[route("/")]
    Presentacion {},
    #[route("/codigo")]
    Codigo {},
    #[route("/wallpapers")]
    Wallpapers {},
    #[route("/estudios")]
    Seccion_estudio_rust {},
    #[route("/libros")]
    Seccion_libros{},
    #[route("/linux")]
    Seccion_linux_general{},
    #[route("/util")]
    Seccion_util{},
    #[route("/musica")]
    Seccion_musica{},
    #[route("/futuro")]
    Seccion_futuro{},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}


#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}

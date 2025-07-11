use dioxus::prelude::*;
#[allow(non_snake_case)]
const MAIN_CSS: Asset = asset!("/assets/main.css");
const IMAGEN_1_PNG: Asset = asset!("/assets/3.png");
const IMAGENES_HTML: Asset = asset!("src/index.html");
fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Presentacion{} 
        EC_Principal{} 
        Seccion_linux_general{} 
    }
}

#[component]
pub fn Presentacion() -> Element {
    rsx!{
        div { id: "rey",
            
            img {id: "Icono", 
                src: "https://avatars.githubusercontent.com/u/136939439?v=4"
            } 
            h1 {"E.C-WEB es mi web personal :v, 
                aca intentare usarlo para recopilar paginas o 
                informacion que me podria ayudar en el futuro!!!!"}
        }
    }
}

#[component]
pub fn EC_Principal() -> Element {
    rsx! {
        div { id: "Ec_prince", 
 
        }         
        div { id: "Texto",  

            h1 {"Seccion util"}
            a {href: "https://developer.mozilla.org/en-US/docs/Web/CSS", 
                h1 {  "CSS EN SU EXPLENDOR" }
            } 
            p {  "Pagina la cual estoy usando para retomar css xD"} 
            a {href: "https://landchad.net/", 
                h1 {  "ChadLandia" }
            } 
            p {  "Pagina que tiene varias weas para aprender de la vida."} 
            a {href: "https://lukesmith.xyz/links/", 
                h1 {  "Luke smith pagina" }
            } 
        
            p { "Esta pagina recopila otras mejores paginas jijiji" } 
        }
        div {  
        }
    }
}


#[component]
pub fn Seccion_linux_general() -> Element {
    rsx! {
        br {}         
        hr { id: "lineas_separar"} 
        div { id: "Texto",  

            h1 {"Seccion linux general"}
            a {href: "https://github.com/realECoficial/dotfiles", 
                h1 {  "Dotfiles personales" }
            } 
            p {  "Aca estan mis configs de hyprland para arch linux."} 
            a {href: "https://nathan.rs/posts/dioxus-rust/#why-rust-for-front-end-development", 
                h1 {  "Frontend Rust" }
            } 
            p {  "Buscando como poner otra web en rust, encontre esta que se ve buena!!11"} 
            
                 
            a {href: IMAGENES_HTML, 
            h1 {  "Fondos de pantalla" }
            } 
            p { "Aca guardo imagenes que encuentro yume 2kki" } 
        }
        div {  
        }
    }
}


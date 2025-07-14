use dioxus::prelude::*;
#[allow(non_snake_case)]
const MAIN_CSS: Asset = asset!("/assets/main.css");
const IMAGEN_1_PNG: Asset = asset!("/assets/3.png");
const IMAGENES_HTML: Asset = asset!("/assets/index.html");

fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Presentacion{} 
        Indice{}
        Seccion_linux_general{} 
        Seccion_util{}  
    }
}
/*
 *TODO usar esto en el final, aun estoy aprendiendo a linkear paginas.
            a {href: IMAGENES_HTML, 
            h1 {  "Fondos de pantalla" }
            } 
            p { "Aca guardo imagenes que encuentro yume 2kki" } 
 * */


#[component]
pub fn Presentacion() -> Element {
    rsx!{
        div { id: "rey",
            
            img { src: "https://avatars.githubusercontent.com/u/136939439?v=4"}, 
            h1 {"E.C-WEB es mi web personal :v, dejare esto como repositorio."}
        }
    }
}


#[component]
pub fn Indice() -> Element {
    rsx!{
        
        br {}         
        div { id: "indice",
            h1 {"ìndice"},
            a {href:"Presentacion", p{"Presentacion"}} 
            a {href:"Seccion_util", p{"Secciòn util"}} 
            
        }
        br {}         
        br {}         
    }
}



#[component]
pub fn Seccion_linux_general() -> Element {
    rsx! {
        br {id: "Presentacion"}         
        hr { id: "lineas_separar"} 
        div { id: "Texto",  

            h1 {"Seccion linux general"}
            a {href: "https://github.com/realECoficial/dotfiles", 
                h2 {  "Dotfiles personales" }
            } 
            a {href: "https://nathan.rs/posts/dioxus-rust/#why-rust-for-front-end-development", 
                h2 {  "Frontend Rust" }
            } 
            
                 
        }
    }
}



#[component]
pub fn Seccion_util() -> Element {
    rsx! {
        br {id: "Seccion_util"}         
        hr { id: "lineas_separar"} 
        div { id: "Texto",  

            h1 {"Seccion util"}
            a {href: "https://www.susanrigetti.com/philosophy", 
                h2 {  "Como aprender filosofia (Susan Rigetti)" }
            } 
            a {href: "https://www.susanrigetti.com/physics", 
                h2 {  "Como aprender fisica (Susan Rigetti)" }
            } 
        
            a {href: "https://missing.csail.mit.edu/", 
                h2 {  "Clases/lectures de M.I.T de herramientas y curiosidades." }
            } 
        }
        
    }
}

/*

#[component]
pub fn Seccion_util() -> Element {
    rsx! {
        br {}         
        hr { id: "lineas_separar"} 
        div { id: "Texto",  

            h1 {"Seccion util"}
            a {href: "https://www.susanrigetti.com/philosophy", 
                h1 {  "Como aprender filosofia (Susan Rigetti)" }
            } 
            p {  "Filosofia."} 
            a {href: "https://www.susanrigetti.com/physics", 
                h1 {  "Como aprender fisica (Susan Rigetti)" }
            } 
            p {  "Fisica."} 
        }
        
    }
}


*/



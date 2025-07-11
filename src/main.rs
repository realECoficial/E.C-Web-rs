use dioxus::prelude::*;
#[allow(non_snake_case)]
const MAIN_CSS: Asset = asset!("/assets/main.css");
fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Presentacion{} 
        EC_Principal{} 
    }
}

#[component]
pub fn Presentacion() -> Element {
    rsx!{
        div { id: "rey",
            
            img {id: "Icono", 
                src: "https://avatars.githubusercontent.com/u/136939439?v=4"
            } 
            h1 {"E.C-WEB es mi web personal :v, aca intentare usarlo para recopilar paginas o informacion que me podria ayudar en el futuro!!!!"}
        }
    }
}

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

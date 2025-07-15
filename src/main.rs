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
        Seccion_musica{}
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
            h1 {"ìndice"}
            a {href:"#linux", p{"Secciòn linux general"}} 
            a {href:"#Seccion_util", p{"Secciòn util"}} 
            a {href:"#musica",       p{"Musica"}} 
        }
        br {}         
        br {}         
    }
}



#[component]
pub fn Seccion_linux_general() -> Element {
    rsx! {
        br {id: "linux"}         
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


#[component]
pub fn Seccion_musica() -> Element {
    let mut count = String::from("");


    rsx! {
        br {id:"musica"}         
        hr { id: "lineas_separar"} 

        div { id: "Texto",  

            h1 {"DISCLAIMER: quisiera que no utilizen plataformas de streaming(pagadas o no), de musica. Todas esas aplicaciones son de control mental."}
            h2 {"Si quieren escuchar musica les recomiendo que la descarguen o se la compren,"}
            h2 {"le da valor agregado."} 
            br {}         
                 
            a { id: "yamete",href: "https://y2mate.as/en-NO0b/", 
                h1 {  "PAGINA PARA DESCARGAR YOUTUBE." }
            } 
            a {href: "https://www.youtube.com/watch?v=Kh3L7u7yuyA", 
                h1 {  "save file 2" }
                
            } 
            
            a {href: "https://www.youtube.com/watch?v=1Q0Fd66kgZM", 
                h1 {  "Alaska - The Vortex / Invisible" }
            } 
            a {href: "https://www.youtube.com/watch?v=a_6quQ994JI", 
                h1 {  "Sōtaisei Riron ( 相対性理论)-Synchroniciteen (Full Album)" }
            } 
            
            a {href: "https://www.youtube.com/watch?v=ODysC7SM_Yk", 
                h1 {  "相対性理論 - 気になるあの娘" }
            } 
            a {href: "https://www.youtube.com/watch?v=XOgFYjwEopo", 
                h1 {  "rosenbridge" }
            } 
            a {href: "https://www.youtube.com/watch?v=Q6YV_rpQ4Jk&list=PLB80A16AFA79B0379&index=3", 
                h1 {  "Yu-Gi-Oh! Ultimate Masters Edition 2006 OST - Specials" }
            } 
            a {href: "https://www.youtube.com/watch?v=VEUZGwwP0FY&list=PLB80A16AFA79B0379&index=9", 
                h1 {  "Yu-Gi-Oh! Ultimate Masters Edition 2006 OST - Level 2 Monster" }
            } 
            a {href: "https://www.youtube.com/watch?v=gpyuAT9q06c", 
                h1 {  "rocket coaster" }
            } 
            a {href: "https://www.youtube.com/watch?v=wHNSSbGDrfo", 
                h1 {  "Owain Panchiko (DEATHMETAL Remix)" }
            } 
            a {href: "https://www.youtube.com/watch?v=DuWQk4eA3lU", 
                h1 {  "'Linebreak' (Amiga .mod!)" }
            } 
        }
        
    }
}





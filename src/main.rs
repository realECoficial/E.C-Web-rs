use dioxus::prelude::*;
#[allow(non_snake_case)]
const MAIN_CSS: Asset = asset!("/assets/main.css");
const IMAGEN_1_PNG: Asset = asset!("/assets/3.png");
const IMAGENES_HTML: Asset = asset!("/assets/index.html");
//libros
const MARCUS: Asset = asset!("/assets/libros/Marcus-Aurelius-Meditations.pdf");
const LIBRO1: Asset = asset!("/assets/libros/25 Roberto Bolaño - Los detectives salvajes.pdf");
const LIBRO2: Asset = asset!("/assets/libros/Indigno-de-ser-humano.pdf");
//musica
const MUSIC1: Asset = asset!("/assets/musica/Ocean girl _ perfect world.mp3");
const MUSIC2: Asset = asset!("/assets/musica/65_saves.mp3");
const MUSIC3: Asset = asset!("/assets/musica/tasty_trugictra.mp3");
const MUSIC4: Asset = asset!("/assets/musica/Substance - Them Phibez.mp3");
const MUSIC5: Asset = asset!("/assets/musica/NoRedeemingQualities.mp3");
const MUSIC6: Asset = asset!("/assets/musica/YOU.mp3");
const MUSIC7: Asset = asset!("/assets/musica/Madwreck-Ride.mp3");
const MUSIC8: Asset = asset!("/assets/musica/海神-Watazumi-.mp3");


fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Presentacion{} 
        Indice{}
        Seccion_estudio_rust{}
        Seccion_libros{}
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
            h1 {"Lo estare usando para trackear donde voy en mis estudios de:"}
            h2 {"Rust"}
            h2 {"Filosofia"}
        }
    }
}


#[component]
pub fn Indice() -> Element {
    rsx!{
        
        br {}         
        div { id: "indice",
            h1 {"ìndice"}
        
            a {href:"#estudios",       p{"Estudios Generales"}} 
            a {href:"#libros",       p{"Libros que recomiendo"}} 
            a {href:"#linux",       p{"Secciòn linux general"}} 
            a {href:"#Seccion_util", p{"Secciòn util"}} 
            a {href:"#musica",       p{"Musica"}} 
        }
        br {}         
        br {}         
    }
}


#[component]
pub fn Seccion_estudio_rust() -> Element {
    rsx! {
        br {id: "estudios"}         
        hr { id: "lineas_separar"} 
        div { id: "Texto",  

            h1 {"Seccion Estudios Generales"}
            a {href: "https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html", 
                h2 {  "Rust." }
            } 
            p {"Estoy en la parte de modulos de Rust, no me gusta mucho XD."} 
             
            
            a {href: MARCUS , 
                h2 {  "Filosofia, Marcus Aurelius: Meditations (Pag 25)." }
            } 
            p {"Me encanta pero me pierdo en el ingles antiguo xD, voy a intentar hacer lo que dice " a {href:"https://www.susanrigetti.com/philosophy", p{"en la seccion 'How to study'."}}} 
             
                 
        }
    }
}


#[component]
pub fn Seccion_libros() -> Element {
    rsx! {
        br {id: "libros"}         
        hr { id: "lineas_separar"} 
        div { id: "Texto",  

            h1 {"Seccion Libros que recomiendo"}
            a {href: LIBRO1, 
                h2 {  "Roberto Bolaño - Los detectives salvajes" }
            } 
             
            a {href: MARCUS , 
                h2 {  "Marcus Aurelius - Meditations." }
            } 
             
                 
            a {href: LIBRO2, 
                h2 {  "Osamu Dazai - Indigno de ser humano" }
            } 
        }
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
            h2 {"(Esta primera parte se puede escuchar aca mismo, despues es todo youtube.)"} 
            br {}         
                
/* IGNORAR, es para no ir para arriba xd            
const MUSIC1: Asset = asset!("/assets/musica/Ocean girl _ perfect world.mp3");
const MUSIC2: Asset = asset!("/assets/musica/65_saves.mp3");
const MUSIC3: Asset = asset!("/assets/musica/tasty_trugictra.mp3");
const MUSIC4: Asset = asset!("/assets/musica/Substance - Them Phibez.mp3");
const MUSIC5: Asset = asset!("/assets/musica/NoRedeemingQualities.mp3");
const MUSIC6: Asset = asset!("/assets/musica/YOU.mp3");
const MUSIC7: Asset = asset!("/assets/musica/Madwreck-Ride.mp3");
const MUSIC8: Asset = asset!("/assets/musica/海神-Watazumi-.mp3");
*/
            
            a {href: MUSIC1, 
                h1 {  "Ocean_girl - perfect world." }
            } 
            
            a {href: MUSIC2, 
                h1 {  "65 save" }
            } 
            
            a {href: MUSIC3, 
                h1 {  "Trugictra - Tasty (Fastracker)" }
            } 

            a {href: MUSIC4, 
                h1 {  "Substance - Them Phibez" }
            } 
            
            a {href: MUSIC5, 
                h1 {  "40k! - NoRedeemingQualities" }
            } 
            a {href: MUSIC6, 
                h1 {  "Harito - YOU" }
            } 

            a {href: MUSIC7, 
                h1 {  "Madwreck - Ride (Fastracker)" }
            } 
            a {href: MUSIC8, 
                h1 {  "海神 - Watazumi" }
            } 
            br {}         
            h1 {"Esto para abajo es de youtube, TODO meter toda la musica xd."} 
            

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





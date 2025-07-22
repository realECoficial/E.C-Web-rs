use dioxus::prelude::*;
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

const FOTOCODIGO: Asset = asset!("/assets/codigos/codewars.png");
const CODIGO: Asset = asset!("/assets/codigos/codigo.txt");

const IMAGEN_1_PNG: Asset = asset!("/assets/wallpapers/1.png");
const IMAGEN_2_PNG: Asset = asset!("/assets/wallpapers/2.png");
const IMAGEN_3_PNG: Asset = asset!("/assets/wallpapers/3.png");
const IMAGEN_4_PNG: Asset = asset!("/assets/wallpapers/4.png");
const IMAGEN_5_PNG: Asset = asset!("/assets/wallpapers/5.png");
const IMAGEN_6_PNG: Asset = asset!("/assets/wallpapers/6.png");
const IMAGEN_7_PNG: Asset = asset!("/assets/wallpapers/7.png");
const IMAGEN_8_PNG: Asset = asset!("/assets/wallpapers/8.png");
const IMAGEN_9_PNG: Asset = asset!("/assets/wallpapers/9.png");
const IMAGEN_10_PNG: Asset = asset!("/assets/wallpapers/10.png");
const IMAGEN_11_PNG: Asset = asset!("/assets/wallpapers/11.png");



pub fn Codigo() -> Element {
    rsx!{
        h1 {"ACA PONDRE CODIGO QUE ME COMPETE XD"}
        img {src: FOTOCODIGO} 
        a {href: CODIGO, p{"codigo"}}
    }

}

pub fn Wallpapers() -> Element {
    rsx!{
        h1 {"Wallpapers que tengo en el pc :p"}
        div { id: "wallpapers",

            img {src: IMAGEN_1_PNG} 
            img {src: IMAGEN_2_PNG} 
            img {src: IMAGEN_3_PNG} 
            img {src: IMAGEN_4_PNG} 
            img {src: IMAGEN_5_PNG} 
            img {src: IMAGEN_6_PNG} 
            img {src: IMAGEN_7_PNG} 
            img {src: IMAGEN_8_PNG} 
            img {src: IMAGEN_9_PNG} 
            img {src: IMAGEN_10_PNG} 
            img {src: IMAGEN_11_PNG} 
        } 
        //a {href: CODIGO, p{"codigo"}}
    }

}

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
            a {href:"#futuro",       p{"Futuro"}} 
        }
        br {}         
        br {}         
    }
}


#[component]
pub fn Seccion_estudio_rust() -> Element {
    rsx! {
        br {id: "estudios"}         
        //hr { id: "lineas_separar"} 
        div { id: "Texto",  

            h1 {"Seccion Estudios Generales"}
            a {href: "https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html", 
                h2 {  "Rust." }
            } 
            p {"Me aburri de aprender asi que estoy practicando aca:", a {href: "https://www.codewars.com/kata/5899642f6e1b25935d000161/train/rust", p{"codewars"}}} 
            p {"..."} 
             
            
            a {href: "https://dioxuslabs.com/learn/0.6/guide/routing/", 
                h2 {  "YA TERMINE LA WEA XDDD" }
            } 
           
            a {href: "https://dioxuslabs.com/learn/0.6/router/example/first-route/", 
                h2 {  "Dioxus: First Route." }
            } 
            a {href: MARCUS , 
                h2 {  "Filosofia, Marcus Aurelius: Meditations (Pag 38)." }
            } 
            p {"Me encanta pero me pierdo en el ingles antiguo xD, voy a intentar hacer lo que dice " a {href:"https://www.susanrigetti.com/philosophy", p{"en la seccion 'How to study'."}}} 
             
                 
        }
    }
}


#[component]
pub fn Seccion_libros() -> Element {
    rsx! {
        br {id: "libros"}         
        //hr { id: "lineas_separar"} 
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
        //hr { id: "lineas_separar"} 
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
        //hr { id: "lineas_separar"} 
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
        //hr { id: "lineas_separar"} 

        div { id: "Texto",  

            h1 {"DISCLAIMER: quisiera que no utilizen plataformas de streaming(pagadas o no), de musica. Todas esas aplicaciones son de control mental."}
            h2 {"Si quieren escuchar musica les recomiendo que la descarguen o se la compren,"}
            h2 {"le da valor agregado."} 
            h2 {"(Esta primera parte se puede escuchar aca mismo, despues es todo youtube.)"} 
            br {}         
                
            a { id: "yamete",href: "https://www.music-map.de/", 
            h1 {  "PAGINA PARA VER MAPA HISTORICO DE LAS BANDAS QUE ESCUCHAS." }
            } 
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
                h1 {  "Susquatch - awakening at daybreak" }
                
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


#[component]
pub fn Seccion_futuro() -> Element {
    rsx! {
        br {id: "futuro"}         
        //hr { id: "lineas_separar"} 
        div { id: "Texto",  

            h1 {"Futuro"}
            a {href: "https://www.portalinmobiliario.com/venta/casa/maule-maule/10371-el-portal-de-talca-2-nva#polycard_client=recommendations_classi-portalinmobiliario-vip&reco_backend=realestate-odin&reco_model=organicos-vis&reco_client=classi-portalinmobiliario-vip&reco_item_pos=7&reco_backend_type=function&reco_id=efd7a643-2ab3-48be-9abc-d5d841e0baab", 
                h2 {  "Quiero vivir en algo asi, aunque quiero aislarme màs." }
            } 
        }
        
    }
}


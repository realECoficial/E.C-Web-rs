use dioxus::prelude::*;
use crate::Route;
#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { id: "title",
            Link { to: Route::Presentacion {},
                h3 { "Principal" }
            }
            Link { to: Route::Seccion_estudio_rust {}, 
                h3 {"Estudios"}
            } // <------- add this Link
            Link { to: Route::Codigo {}, 
                h3 {"Codigo :v"}
            } // <------- add this Link
            Link { to: Route::Seccion_linux_general {}, 
                h3 {"Linux"}
            } // <------- add this Link
            Link { to: Route::Seccion_util {}, 
                h3 {"Util"}
            } // <------- add this Link
            Link { to: Route::Seccion_libros {}, 
                h3 {"Libros"}
            } // <------- add this Link
            Link { to: Route::Seccion_musica {}, 
                h3 {"Musica"}
            } // <------- add this Link
            Link { to: Route::Wallpapers {}, 
                h3 {"Wallpapers"}
            } // <------- add this Link
            Link { to: Route::Seccion_futuro  {}, 
                h3 {"Futuro"}
            } // <------- add this Link
        }
        Outlet::<Route> {}
    }
}

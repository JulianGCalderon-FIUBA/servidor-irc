use gtk4 as gtk;

use crate::view_register::RegisterView;
use gtk::{prelude::*, glib::GString};

pub struct RegisterController {
    view: RegisterView
}

impl RegisterController {
    pub fn new(view: RegisterView) -> Self {
        Self {
            view
        }
    }

    pub fn start(&mut self, app: &gtk::Application) {
        self.view.get_view(app).show();
    }

    pub fn login_clicked(&self, pass: GString, nick: GString, username: GString) {
        // println!("Clicked");

        // let mut outchild = Command::new("bash")
        // .arg("PASS hola")
        // .stdin(Stdio::piped())
        // .spawn()
        // .unwrap();

        // outchild.stdin.take().expect("Could not write").write_all(b"Hola").expect("Couldnt write");

        // write!(outchild.stdin.unwrap(), "Hola").unwrap();

        // match Command::new("chroot")
        //     .stdin(Stdio::piped())
        //     .arg("/mnt")
        //     .arg("passwd")
        //     .arg("hola")
        //     .spawn()
        // {
        //     Ok(mut child) => {
        //         let pwd = format!("{}\n{}", "hola123", "hola123");
        //         child.stdin.as_ref().unwrap().write(pwd.as_bytes()).unwrap();
        //         child.wait().unwrap();
        //     }
        //     Err(_e) => (),
        // }

        println!("PASS {}", pass);
        println!("NICK {}", nick);
        println!("USER {} {} {} :{}", username, username, username, username);
        
    }
}

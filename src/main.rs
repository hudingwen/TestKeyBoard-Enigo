use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use enigo::{Enigo, Key, KeyboardControllable};
use std::thread;
use std::time::Duration;
use std::env;



// 定义一个用于处理get请求的handler
#[get("/test/{key}")]
async fn test(web::Path(key): web::Path<String>) -> impl Responder {
    println!("开始任务-test");
    println!("key:{}",key);
    thread::sleep(Duration::from_secs(1));
    let mut enigo = Enigo::new(); 
    enigo.key_down(Key::LeftArrow);
    thread::sleep(Duration::from_secs(1));
    enigo.key_up(Key::LeftArrow);
    let mut enigo = Enigo::new(); 
    enigo.key_down(Key::RightArrow);
    thread::sleep(Duration::from_secs(1));
    enigo.key_up(Key::RightArrow);
    format!("ok")
}
// 定义一个用于处理get请求的handler
#[get("/click/{key}")]
async fn click(web::Path(key): web::Path<String>) -> impl Responder {
    println!("开始任务click");
    println!("key:{}",key);
    // thread::sleep(Duration::from_secs(2));
    let mut enigo = Enigo::new(); 
    
    let mut chars: Vec<_> = key.chars().collect();
    //println!("The character length {}", chars.len());
    if chars.len()==1 {
        enigo.key_down(Key::Layout(chars[0]));
        enigo.key_up(Key::Layout(chars[0]));
    } else if chars.len() > 1{
       if key == "F1" {
            enigo.key_click(Key::F1);
       }else if key == "F2" {
            enigo.key_click(Key::F2);
        }else if key == "F3" {
            enigo.key_click(Key::F3);
        }else if key == "F4" {
            enigo.key_click(Key::F4);
        }else if key == "F5" {
            enigo.key_click(Key::F5);
        }else if key == "F6" {
            enigo.key_click(Key::F6);
        }else if key == "F7" {
            enigo.key_click(Key::F7);
        }else if key == "F8" {
            enigo.key_click(Key::F8);
        }else if key == "F9" {
            enigo.key_click(Key::F9);
        }else if key == "F10" {
            enigo.key_click(Key::F10);
        }else if key == "F11" {
            enigo.key_click(Key::F11);
        }else if key == "F12" {
            enigo.key_click(Key::F12);
        }else if key == "Alt" {
            enigo.key_click(Key::Alt);
        }else if key == "Backspace" {
            enigo.key_click(Key::Backspace);
        }else if key == "CapsLock" {
            enigo.key_click(Key::CapsLock);
        }else if key == "Control" {
            enigo.key_click(Key::Control);
        }else if key == "Delete" {
            enigo.key_click(Key::Delete);
        }else if key == "DownArrow" {
            enigo.key_click(Key::DownArrow);
        }else if key == "End" {
            enigo.key_click(Key::End);
        }else if key == "Escape" {
            enigo.key_click(Key::Escape);
        }else if key == "Home" {
            enigo.key_click(Key::Home);
        }else if key == "LeftArrow" {
            enigo.key_click(Key::LeftArrow);
        }else if key == "Option" {
            enigo.key_click(Key::Option);
        }else if key == "PageDown" {
            enigo.key_click(Key::PageDown);
        }else if key == "PageUp" {
            enigo.key_click(Key::PageUp);
        }else if key == "Return" {
            enigo.key_click(Key::Return);
        }else if key == "RightArrow" {
            enigo.key_click(Key::RightArrow);
        }else if key == "Shift" {
            enigo.key_click(Key::Shift);
        }else if key == "Space" {
            enigo.key_click(Key::Space);
        }else if key == "Tab" {
            enigo.key_click(Key::Tab);
        }else if key == "UpArrow" {
            enigo.key_click(Key::UpArrow);
        }else if key == "Numpad0" {
            enigo.key_click(Key::Numpad0);
        }else if key == "Numpad1" {
            enigo.key_click(Key::Numpad1);
        }else if key == "Numpad2" {
            enigo.key_click(Key::Numpad2);
        }else if key == "Numpad3" {
            enigo.key_click(Key::Numpad3);
        }else if key == "Numpad4" {
            enigo.key_click(Key::Numpad4);
        }else if key == "Numpad5" {
            enigo.key_click(Key::Numpad5);
        }else if key == "Numpad6" {
            enigo.key_click(Key::Numpad6);
        }else if key == "Numpad7" {
            enigo.key_click(Key::Numpad7);
        }else if key == "Numpad8" {
            enigo.key_click(Key::Numpad8);
        }else if key == "Numpad9" {
            enigo.key_click(Key::Numpad9);
        }else if key == "Cancel" {
            enigo.key_click(Key::Cancel);
        }else if key == "Clear" {
            enigo.key_click(Key::Clear);
        }else if key == "Pause" {
            enigo.key_click(Key::Pause);
        }
    }else{
        println!("错误")
    }
    println!("结束任务");
    format!("ok")
}

// 定义一个用于处理get请求的handler
#[get("/press/{key}")]
async fn press(web::Path(key): web::Path<String>) -> impl Responder {
    println!("开始任务-press");
    println!("key:{}",key);
    let mut enigo = Enigo::new(); 
    
    let mut chars: Vec<_> = key.chars().collect();
    //println!("The character length {}", chars.len());
    if chars.len()==1 {
        enigo.key_down(Key::Layout(chars[0]));
        thread::sleep(Duration::from_secs(1));
        enigo.key_up(Key::Layout(chars[0]));
    } else if chars.len() > 1{
       if key == "F1" {
            enigo.key_down(Key::F1);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::F1);
       }else if key == "F2" {
            enigo.key_down(Key::F2);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::F2);
        }else if key == "F3" {
            enigo.key_down(Key::F3);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::F3);
        }else if key == "F4" {
            enigo.key_down(Key::F4);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::F4);
        }else if key == "F5" {
            enigo.key_down(Key::F5);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::F5);
        }else if key == "F6" {
            enigo.key_down(Key::F6);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::F6);
        }else if key == "F7" {
            enigo.key_down(Key::F7);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::F7);
        }else if key == "F8" {
            enigo.key_down(Key::F8);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::F8);
        }else if key == "F9" {
            enigo.key_down(Key::F9);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::F9);
        }else if key == "F10" {
            enigo.key_down(Key::F10);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::F10);
        }else if key == "F11" {
            enigo.key_down(Key::F11);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::F11);
        }else if key == "F12" {
            enigo.key_down(Key::F12);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::F12);
        }else if key == "Alt" {
            enigo.key_down(Key::Alt);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Alt);
        }else if key == "Backspace" {
            enigo.key_down(Key::Backspace);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Backspace);
        }else if key == "CapsLock" {
            enigo.key_down(Key::CapsLock);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::CapsLock);
        }else if key == "Control" {
            enigo.key_down(Key::Control);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Control);
        }else if key == "Delete" {
            enigo.key_down(Key::Delete);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Delete);
        }else if key == "DownArrow" {
            enigo.key_down(Key::DownArrow);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::DownArrow);
        }else if key == "End" {
            enigo.key_down(Key::End);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::End);
        }else if key == "Escape" {
            enigo.key_down(Key::Escape);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Escape);
        }else if key == "Home" {
            enigo.key_down(Key::Home);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Home);
        }else if key == "LeftArrow" {
            enigo.key_down(Key::LeftArrow);
            enigo.key_down(Key::Alt);
            thread::sleep(Duration::from_millis(400));
            enigo.key_up(Key::Alt);
            enigo.key_up(Key::LeftArrow);
        }else if key == "Option" {
            enigo.key_down(Key::Option);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Option);
        }else if key == "PageDown" {
            enigo.key_down(Key::PageDown);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::PageDown);
        }else if key == "PageUp" {
            enigo.key_down(Key::PageUp);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::PageUp);
        }else if key == "Return" {
            enigo.key_down(Key::Return);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Return);
        }else if key == "RightArrow" {
            enigo.key_down(Key::RightArrow);
            enigo.key_down(Key::Alt);
            thread::sleep(Duration::from_millis(300));
            enigo.key_up(Key::Alt);
            enigo.key_up(Key::RightArrow);
        }else if key == "Shift" {
            enigo.key_down(Key::Shift);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Shift);
        }else if key == "Space" {
            enigo.key_down(Key::Space);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Space);
        }else if key == "Tab" {
            enigo.key_down(Key::Tab);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Tab);
        }else if key == "UpArrow" {
            enigo.key_down(Key::UpArrow);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::UpArrow);
        }else if key == "Numpad0" {
            enigo.key_down(Key::Numpad0);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Numpad0);
        }else if key == "Numpad1" {
            enigo.key_down(Key::Numpad1);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Numpad1);
        }else if key == "Numpad2" {
            enigo.key_down(Key::Numpad2);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Numpad2);
        }else if key == "Numpad3" {
            enigo.key_down(Key::Numpad3);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Numpad3);
        }else if key == "Numpad4" {
            enigo.key_down(Key::Numpad4);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Numpad4);
        }else if key == "Numpad5" {
            enigo.key_down(Key::Numpad5);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Numpad5);
        }else if key == "Numpad6" {
            enigo.key_down(Key::Numpad6);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Numpad6);
        }else if key == "Numpad7" {
            enigo.key_down(Key::Numpad7);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Numpad7);
        }else if key == "Numpad8" {
            enigo.key_down(Key::Numpad8);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Numpad8);
        }else if key == "Numpad9" {
            enigo.key_down(Key::Numpad9);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Numpad9);
        }else if key == "Cancel" {
            enigo.key_down(Key::Cancel);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Cancel);
        }else if key == "Clear" {
            enigo.key_down(Key::Clear);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Clear);
        }else if key == "Pause" {
            enigo.key_down(Key::Pause);
            thread::sleep(Duration::from_secs(1));
            enigo.key_up(Key::Pause);
        }
    }else{
        println!("错误")
    }

    println!("结束任务");
    format!("ok")
}

// 定义一个用于处理post请求的handler
#[post("/post")]
async fn post(req_body: String) -> impl Responder {
    format!("Echo: {}", req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //let args: Vec<String> = env::args().collect();

    //let argc = args.len(); //参数数量

    //println!("{}",args[0]); //程序路径，总是存在
    //println!("{}",args[1]); //第一个参数,如果有
    //println!("{}",args[2]); //第二个参数，如果有

    //遍历打印参数索引、值
    // for (index, value) in args.iter().enumerate() {
    //     println!("{} => {}", index,value );
    // }
    // println!("任务开始...");
    // thread::sleep(Duration::from_secs(5));
    // let mut enigo = Enigo::new();
    // enigo.key_click(Key::Layout('a'));
    // thread::sleep(Duration::from_secs(1));
    //enigo.key_click(Key::Layout('d'));
    // println!("点击成功...");
    println!("开启服务...");
    
    HttpServer::new(|| {
        App::new()
            .service(post)
            .service(click)
            .service(press)
            .service(test)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
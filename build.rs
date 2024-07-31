use std::process::Command;

fn main() {
    Command::new("cmd")
        .arg("/C")
        .arg("tailwindcss -i assets/tailwind.css -o assets/main.css --minify")
        .output()
        .expect("Failed to compile tailwind css!");
}

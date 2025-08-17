fn main() {
    // Only run on Windows
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("exec-brute-force.ico"); // point to your .ico file
        res.compile().expect("Failed to compile Windows resources");
    }
}

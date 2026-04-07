pub fn should_keep(line: &str) -> bool {

    //keep important log levels
    if !(line.starts_with("E/") || line.starts_with("W/") || line.starts_with("I/")) {
        return false;
    }

    let noisy_tags = [
        "OpenGLRenderer",
        "Choreographer",
        "BufferQueue",
        "SurfaceView",
        "ViewRootImpl"
    ];


    for tag in noisy_tags {
        if line.contains(tag){
            return false;
        }
    }
    true

}

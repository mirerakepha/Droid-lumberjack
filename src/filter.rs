/*
 */
pub fn should_keep(line: &str) -> bool {
    if line.starts_with("--------- beginning of") {
        return false;
    }
    // Drop pure debug/verbose but keep E, W, I, F
    let has_level = line.contains(" E ")
        || line.contains(" W ")
        || line.contains(" I ")
        || line.contains(" F ");
    if !has_level {
        return false;
    }
    let noisy_tags = [
        "OpenGLRenderer",
        "Choreographer",
        "BufferQueue",
        "EGL_emulation",
        "libEGL",
    ];
    for tag in noisy_tags {
        if line.contains(tag) {
            return false;
        }
    }
    true
}

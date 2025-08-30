pub fn install_ctrlc_handler() {
    let (interrupt_send, interrupt_recv) = std::sync::mpsc::channel();

    ctrlc::set_handler(move || {
        println!("Shutting down...");
        let _ = interrupt_send.send(());
    })
    .expect("Error setting Ctrl-C handler");

    // Monitor for interrupt signals in a separate thread
    // This is necessary because we can't directly exit from the signal handler.

    std::thread::spawn(move || {
        if interrupt_recv.recv().is_ok() {
            // Exit with code 130 (128 + SIGINT's signal number 2)
            // This is the standard Unix convention for processes terminated by SIGINT
            std::process::exit(130);
        }
    });
}

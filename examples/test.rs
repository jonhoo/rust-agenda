extern crate agenda;

fn main() {
    let p = agenda::start();
    let p = p.enter("Set up network");
    p.single("Create Virtual Private Cloud");
    p.single("Attach internet gateway");
    let p = p.enter("Allocate subnet #1");
    p.single("Hook in internet-enabled route table");
    let p = p.leave();
    p.single("Allocate subnet #2");
    let p = p.enter("Generate VPC key-pair");
    p.error("Could not create key-pair");
    p.single("Attempting to delete old key-pair");
    p.single("Attempting to generate new key-pair");
    let p = p.root();

    let p = p.enter("Launch instances");
    p.single("Launch instances in cluster #1");
    p.single("Launch instances in cluster #2");
    let p = p.enter("Wait for HQ to start running");
    p.single("Still in 'pending' state");
    p.single("Still in 'pending' state");
    let p = p.leave();
    let p = p.enter("Wait for workers to reach 'running' state");
    let p = p.enter("Wait for HQ to become pingable");
    eprintln!("54.84.179.156 | UNREACHABLE!");
    eprintln!("54.84.179.156 | UNREACHABLE!");
    eprintln!("54.84.179.156 | SUCCESS => {{\"changed\": false, \"ping\": \"pong\"}}");
    let p = p.leave();
    let p = p.enter("Wait for workers to become pingable");
    eprintln!("10.0.1.237 | SUCCESS => {{\"changed\": false, \"ping\": \"pong\"}}");
    let p = p.root();

    let p = p.enter("Deploy application");
    eprintln!("PLAY [ansible-playbook]");
    eprintln!("TASK [common : Run application]");
    eprintln!("fatal: [10.0.1.237]: FAILED! => {{\"changed\": true, \"cmd\": ...}}");
    eprintln!("PLAY RECAP");
    eprintln!("10.0.1.237: ok=6    changed=4    unreachable=0    failed=1");
    p.error("Application failed to run correctly");
    let p = p.root();
}

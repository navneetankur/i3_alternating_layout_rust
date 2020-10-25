extern crate i3ipc;
use i3ipc::I3EventListener;
use i3ipc::Subscription;
use i3ipc::event::Event;
use i3ipc::I3Connection;
use i3ipc::reply::Node;
use i3ipc::reply::NodeLayout;

fn main() {
    let mut listener = I3EventListener::connect().unwrap();
    let mut connection = I3Connection::connect().unwrap();

    let subs = [Subscription::Window];
    listener.subscribe(&subs).unwrap();

    // handle them
    for event in listener.listen() {
        match event.unwrap() {
            Event::ModeEvent(e) => println!("new mode: {}", e.change),
            Event::BindingEvent(e) => println!("user input triggered command: {}", e.binding.command),
            Event::WindowEvent(e) => {
                if e.change == i3ipc::event::inner::WindowChange::Focus {
                    let tree = connection.get_tree().unwrap();
                    let paren = find_parent(&tree,e.container.id);
                    if paren.is_some() {
                        let parent = paren.unwrap();

                        if parent.layout == NodeLayout::SplitH || parent.layout == NodeLayout::SplitV {
                            //width > height
                            if e.container.rect.2 > e.container.rect.3 {
                                connection.run_command("split h");
                            }
                            else {
                                connection.run_command("split v");
                            }
                        }
                    }
                }
            },
            _ => unreachable!()
        }
    }
}

fn find_parent(current: &Node, window_id: i64) -> Option<&Node>{
    for node in &current.nodes {
        if node.id == window_id {
            return Some(current);
        }
        else {
            let ret = find_parent(&node, window_id);
            if ret.is_some() {
                return ret;
            }
        }
    }
    return None;
}


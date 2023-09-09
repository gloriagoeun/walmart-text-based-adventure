struct Aisle {
    name: String, // E.g. "Soup Aisle"
    desc: String, // E.g. "Dark wood paneling covers the walls.  The gilded northern Pointerway lies open."
    Pointers: Vec<Pointer>,
}
struct Pointer {
    target: AisleID,         // More about this in a minute
    triggers: Vec<String>,   // e.g. "left" or "right" or "straight"
    message: Option<String>, // What message, if any, to print when the Pointerway is traversed
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct AisleID(usize);

fn main() {
    use std::io;
    // We need the Write trait so we can flush stdout
    use std::io::Write;
    use std::time::{Duration, Instant};
    let Aisles = [
        Aisle {
            name: "Cashier".into(), // Turn a &'static string (string constant) into a String
            desc: "(o_o)".into(),
            Pointers: vec![
                Pointer {
                    target: AisleID(1),
                    triggers: vec!["left".into()],
                    message: None,
                },
                Pointer {
                    target: AisleID(2),
                    triggers: vec!["straight".into()],
                    message: None,
                },
                Pointer {
                    target: AisleID(3),
                    triggers: vec!["right".into()],
                    message: None,
                },
            ],
        },
        Aisle {
            name: "Aisle 1".into(),
            desc: "Blub blub blub (o3o)".into(),
            Pointers: vec![
                Pointer {
                    target: AisleID(2),
                    triggers: vec!["right".into()],
                    message: None,
                },
                Pointer {
                    target: AisleID(4),
                    triggers: vec!["straight".into()],
                    message: None,
                },
            ],
        },
        Aisle {
            name: "Aisle 2".into(),
            desc: "Cucumbers (._.  *)".into(),
            Pointers: vec![
                Pointer {
                    target: AisleID(3),
                    triggers: vec!["right".into()],
                    message: None,
                },
                Pointer {
                    target: AisleID(6),
                    triggers: vec!["straight".into()],
                    message: None,
                },
            ],
        },
        Aisle {
            name: "Aisle 3".into(),
            desc: "Cucumbers (._.  *)".into(),
            Pointers: vec![Pointer {
                target: AisleID(4),
                triggers: vec!["left".into()],
                message: None,
            }],
        },
        Aisle {
            name: "Aisle 4".into(),
            desc: "Cucumbers (._.  *)".into(),
            Pointers: vec![Pointer {
                target: AisleID(5),
                triggers: vec!["right".into()],
                message: None,
            }],
        },
        Aisle {
            name: "Aisle 5".into(),
            desc: "Cucumbers (._.  *)".into(),
            Pointers: vec![Pointer {
                target: AisleID(6),
                triggers: vec!["straight".into()],
                message: None,
            }],
        },
        Aisle {
            name: "Aisle 6".into(),
            desc: "Cucumbers (._.  *)".into(),
            Pointers: vec![Pointer {
                target: AisleID(7),
                triggers: vec!["straight".into()],
                message: None,
            }],
        },
        Aisle {
            name: "Aisle 7".into(),
            desc: "Victory".into(),
            Pointers: vec![
                Pointer {
                    target: AisleID(8),
                    triggers: vec!["left".into()],
                    message: None,
                },
                Pointer {
                    target: AisleID(9),
                    triggers: vec!["straight".into()],
                    message: None,
                },
                Pointer {
                    target: AisleID(10),
                    triggers: vec!["right".into()],
                    message: None,
                },
            ],
        },
        Aisle {
            name: "Aisle 8".into(),
            desc: "Bananas".into(),
            Pointers: vec![
                Pointer {
                    target: AisleID(9),
                    triggers: vec!["right".into()],
                    message: None,
                },
                Pointer {
                    target: AisleID(11),
                    triggers: vec!["straight".into()],
                    message: None,
                },
            ],
        },
        Aisle {
            name: "Aisle 9".into(),
            desc: "Clothing".into(),
            Pointers: vec![
                Pointer {
                    target: AisleID(10),
                    triggers: vec!["right".into()],
                    message: None,
                },
                Pointer {
                    target: AisleID(12),
                    triggers: vec!["straight".into()],
                    message: None,
                },
            ],
        },
        Aisle {
            name: "Aisle 10".into(),
            desc: "Trees".into(),
            Pointers: vec![
                Pointer {
                    target: AisleID(12),
                    triggers: vec!["left".into()],
                    message: None,
                },
                Pointer {
                    target: AisleID(13),
                    triggers: vec!["straight".into()],
                    message: None,
                },
            ],
        },
        Aisle {
            name: "Aisle 11".into(),
            desc: "fwevbi ".into(),
            Pointers: vec![Pointer {
                target: AisleID(15),
                triggers: vec!["straight".into()],
                message: None,
            }],
        },
        Aisle {
            name: "Aisle 12".into(),
            desc: "Greetings".into(),
            Pointers: vec![
                Pointer {
                    target: AisleID(11),
                    triggers: vec!["straight".into()],
                    message: None,
                },
                Pointer {
                    target: AisleID(8),
                    triggers: vec!["left".into()],
                    message: None,
                },
            ],
        },
        Aisle {
            name: "Aisle 13".into(),
            desc: "Dinos".into(),
            Pointers: vec![
                Pointer {
                    target: AisleID(12),
                    triggers: vec!["left".into()],
                    message: None,
                },
                Pointer {
                    target: AisleID(14),
                    triggers: vec!["right".into()],
                    message: None,
                },
                Pointer {
                    target: AisleID(15),
                    triggers: vec!["straight".into()],
                    message: None,
                },
            ],
        },
        Aisle {
            name: "Aisle 14".into(),
            desc: "White Paint".into(),
            Pointers: vec![Pointer {
                target: AisleID(13),
                triggers: vec!["straight".into()],
                message: None,
            }],
        },
        Aisle {
            name: "Aisle 15".into(),
            desc: "Cashier".into(),
            Pointers: vec![],
        },
    ];
    let end_Aisles = [AisleID(15)];
    let mut input = String::new();

    let mut at = AisleID(0);
    println!("The Walmart Shopping Adventure");
    println!("============================");
    println!();
    println!("You've been walking for hours in the countryside, and have finally stumbled on the spooky mansion you read about in the tour guide.");
    let start = Instant::now();
    println!("{:?}", start);
    loop {
        // We don't want to move out of Aisles, so we take a reference
        let here = &Aisles[at.0];
        println!("{}\n{}", here.name, here.desc);
        if end_Aisles.contains(&at) {
            println!("You reached the end. You can go home with your parents :)");
            break;
        }
        let mut new_now = Instant::now();

        //the game should end when the time ends
        println!("{:?}", new_now);
        if (new_now.duration_since(start) >= Duration::from_secs(2)) {
            println!("You ran out of time :( Your parents left you..... Sriracha blood is all over you >:))");
            break;
        }
        println!("{:?}", new_now.duration_since(start));
        loop {
            print!("What will you do?\n> ");
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if let Some(Pointer) = here
                .Pointers
                .iter()
                .find(|d| d.triggers.iter().any(|t| *t == input))
            {
                if let Some(msg) = &Pointer.message {
                    println!("{}", msg);
                }
                at = Pointer.target;
                break;
            } else {
                println!("You can't do that!");
            }
        }
    }
}

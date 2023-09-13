extern crate rodio;
use std::io;
// We need the Write trait so we can flush stdout
use rodio::Sink;
use std::fs::File;
use std::io::{BufReader, Write};
use std::thread::sleep;
use std::time::{Duration, Instant}; // Import the Duration type from std::time

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

fn play_mp3_file(file_path: &str, intro_time: &u64) {
    // Open the MP3 file and create a buffer for it.

    let file = File::open(file_path).expect("Failed to open MP3 file");
    let source = rodio::Decoder::new(BufReader::new(file)).expect("Failed to decode MP3 file");

    // Create an audio sink (output) and play the audio.
    let device = rodio::default_output_device().expect("No output device found");
    let sink = Sink::new(&device);

    // Add the decoded audio to the sink.
    sink.append(source);

    // Sleep for a while to allow the audio to play.
    sleep(Duration::from_secs(*intro_time)); // Adjust the duration as needed

    // Stop the playback when you're done.
    sink.stop();
}

fn main() {
    let Aisles = [
        Aisle {
            name: "Cashier".into(), // Turn a &'static string (string constant) into a String
            desc: "Apple, Orange, Banana".into(),
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
            desc: "(o3o)".into(),
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
    let mut input_one = String::new();

    let mut at = AisleID(0);
    println!("The Walmart Shopping Adventure");
    println!("=======================================================");
    println!("Welcome to...");
    println!();
    println!(
        "    :::       :::     :::     :::        ::::    ::::      :::     :::::::::  ::::::::::: 
    :+:       :+:   :+: :+:   :+:        +:+:+: :+:+:+   :+: :+:   :+:    :+:     :+:     
    +:+       +:+  +:+   +:+  +:+        +:+ +:+:+ +:+  +:+   +:+  +:+    +:+     +:+     
    +#+  +:+  +#+ +#++:++#++: +#+        +#+  +:+  +#+ +#++:++#++: +#++:++#:      +#+     
    +#+ +#+#+ +#+ +#+     +#+ +#+        +#+       +#+ +#+     +#+ +#+    +#+     +#+     
     #+#+# #+#+#  #+#     #+# #+#        #+#       #+# #+#     #+# #+#    #+#     #+#     
      ###   ###   ###     ### ########## ###       ### ###     ### ###    ###     ### "
    );
    println!();
    println!();
    println!("You are grocery shopping with your parents!! WOW, so fun...");
    //sleep(Duration::from_secs(2));
    println!("Except, you are stressed.. Why may you ask? Your parents are checking out their items, but THEY FORGOT THE SRIRACHA!!");
    println!("Your mission for this game is to grab the sriracha before the cashier checks out all of the items (and before your parents leave..... without you.... :'()");
    println!("Ready....? The game shall start soon...");

    //playing the intro music here
    let intro_time: u64 = 2;
    let mp3_file_path = "beep.mp3";
    play_mp3_file(mp3_file_path, &intro_time);

    let start = Instant::now();
    let time_limit: u64 = 240;

    loop {
        // We don't want to move out of Aisles, so we take a reference
        let here = &Aisles[at.0];

        //prints the aisle name and description
        println!("{}\n{}", here.name, here.desc);

        if end_Aisles.contains(&at) {
            println!("You reached the end. You can go home with your parents :)");
            break;
        }
        let mut new_now = Instant::now();

        //the game should end when the time ends
        if new_now.duration_since(start) >= Duration::from_secs(time_limit) {
            println!("You ran out of time :( Your parents left you..... Sriracha blood is all over you >:))");
            break;
        }
        //loop {
        println!(
            "You have {:?} seconds left!!!",
            Duration::from_secs(time_limit) - new_now.duration_since(start)
        );
        println!();

        let mut keep = true;

        if at == AisleID(0)
            || at == AisleID(1)
            || at == AisleID(2)
            || at == AisleID(3)
            || at == AisleID(4)
            || at == AisleID(5)
            || at == AisleID(6)
            || at == AisleID(7)
        {
            loop {
                println!("Is the sriracha in this aisle?");
                println!("'Yes' or 'No'? ");
                io::stdout().flush().unwrap();
                input_one.clear();
                io::stdin().read_line(&mut input_one).unwrap();
                let input_one = input_one.trim();
                //if (at == )
                if input_one == "No" {
                    if at == AisleID(7) {
                        at = AisleID(0);
                        println!("You missed it... back to the cashier");
                        keep = false;
                    }
                    break;
                } else if input_one == "Yes" {
                    // say yes
                    if at != AisleID(7) {
                        at = AisleID(0);
                        println!("You missed it... back to the cashier");
                        keep = false;
                    } else {
                        println!("You found it.. Navigate your way back to the cashier!");
                    }
                    break;
                } else {
                    println!("You can't do that!");
                }
            }
        }

        if keep {
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
        //}
    }
}

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
    desc: String,
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

fn show_moves(aisle: &Aisle) -> String {
    let mut res: String = "".to_owned();
    for dir in &aisle.Pointers {
        if let Some(point) = dir.triggers.first() {
            res.push_str(point);
            res.push_str(", ");
        }
    }
    println!("{}", res);
    return res;
}

fn main() {
    let Aisles = [
        Aisle {
            name: "(ಠ_ಠ) Cashier".into(), // Turn a &'static string (string constant) into a String
            desc: " 
            xxxxxxxxxxxxxxxxxxx
            🗣️👥📠⏳❗️🛒 🗣️👥📠⏳
            🗣️👥📠⏳❗️🛒 🗣️👥📠⏳
            🗣️👥📠⏳❗️🛒 🗣️👥📠⏳
            xxxxxxxxxxxxxxxxxxx
            "
            .into(),
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
            name: "Aisle BOM".into(),
            desc: " 
            xxxxxxxxxxxxxxxxxxx
            🍎🍏🍏🍏🍎🍒🍏🍏🍏🍏
            🥫🍎🍎🍒🍎🍎🍎🍏🍎🥫
            🍎🍏🍏🍎🍎🍎🍎🍒🍠🍎            
            xxxxxxxxxxxxxxxxxxx
            "
            .into(),
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
            name: "Aisle BAS".into(),
            desc: " 
            xxxxxxxxxxxxxxxxxxx
            🍈🍎🥝🥝🍈🍠🥝🥑🥑
            🥝🥑🥑🥝🥝🥝🥝🍠🥝
            🥝🥝🥝❗️🥝🍎🥝🥑🥑                       
            xxxxxxxxxxxxxxxxxxx
            "
            .into(),
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
            name: "Aisle TIC".into(),
            desc: " 
            xxxxxxxxxxxxxxxxxxx
            🍠🍑🍊🍑🍊🍊🍑🍊🍊
            🍊🍊🍊🍠🍊🍋🍋🍊🍊
            🍊🍑🍋🍋🍊🍊🍠🍊🍊                       
            xxxxxxxxxxxxxxxxxxx
            "
            .into(),
            Pointers: vec![Pointer {
                target: AisleID(4),
                triggers: vec!["left".into()],
                message: None,
            }],
        },
        Aisle {
            name: "Aisle SIDE".into(),
            desc: " 
            xxxxxxxxxxxxxxxxxxx
            🍅🍓🥫🥫🥫🍒🍒🍅🍅
            🍒🍅🍅🍅🍓🥫🍅🍠🍅
            🍉🥫🥫🥫🥫🍅🍅🍅🍉                                   
            xxxxxxxxxxxxxxxxxxx
            "
            .into(),
            Pointers: vec![Pointer {
                target: AisleID(5),
                triggers: vec!["right".into()],
                message: None,
            }],
        },
        Aisle {
            name: "Aisle EYE".into(),
            desc: " 
            xxxxxxxxxxxxxxxxxxx
            ⏰🥫🖍🎈🧲🧯🧰🌰🚨
            🍒🍅🍅⏰🧶🦀🦐🍠🥕
            🍑☄️🔥🥫🍄🍄🦀🦐🥩                                              
            xxxxxxxxxxxxxxxxxxx
            "
            .into(),
            Pointers: vec![Pointer {
                target: AisleID(6),
                triggers: vec!["straight".into()],
                message: None,
            }],
        },
        Aisle {
            name: "Aisle WOA".into(),
            desc: " 
            xxxxxxxxxxxxxxxxxxx
            🥯🥨🥫🍎🍒🍒🌽🫑🧄
            🍒🍅🍅🥯👶🧀🥖🍠🥕
            🍑🥭🍍🥫🍦🍐🍊🍋🍌                                                    
            xxxxxxxxxxxxxxxxxxx
            "
            .into(),
            Pointers: vec![Pointer {
                target: AisleID(7),
                triggers: vec!["straight".into()],
                message: None,
            }],
        },
        Aisle {
            name: "Aisle TIK".into(),
            desc: " 
            xxxxxxxxxxxxxxxxxxxxxx
            🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️
            🌶️ 💲®️ ℹ️ ®️ 🅰️ ©️ ♓🅰️ 🌶️
            🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️🌶️                                                          
            xxxxxxxxxxxxxxxxxxxxxx
            "
            .into(),
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
            name: "Aisle TOK".into(),
            desc: " 
            xxxxxxxxxxxxxxxxxxx
            ⏰🖨️🎧⏰🖨️🎧🖨️🎧
            ⏰🖨️🎧⏰🖨️🎧⏰🎧
            ⏰🖨️🎧⏰🖨️🎧⏰🖨️                                                                       
            xxxxxxxxxxxxxxxxxxx
            "
            .into(),
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
            name: "Aisle CLICK".into(),
            desc: " 
            xxxxxxxxxxxxxxxxxxx
            🖥️💻📱⏰🖥️💻📱🖥️
            🖥️💻📱⏰🖥️💻📱💻
            🖥️💻📱⏰🖥️💻📱📱                                                                                 
            xxxxxxxxxxxxxxxxxxx
            "
            .into(),
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
            name: "Aisle CLOCK".into(),
            desc: " 
            xxxxxxxxxxxxxxxxxxx
            🖥️🛹🏸⚽️🖥️🛹🏸⚽️
            🖥️🛹🏸⚽️🛹🏸⚽️💻
            🛹⚽️📱🛹🏸⚽️🖥️🏸                                                                                            
            xxxxxxxxxxxxxxxxxxx
            "
            .into(),
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
            name: "Aisle GO".into(),
            desc: " 
            xxxxxxxxxxxxxxxxxxx
            🎂🥮🎂🎂🎂🥮🥮🥮🥮
            🥮🎂🎂🎂🥐🥐🥐🥐🥐
            🥐🥐🥐🥖🥖🥖🥖🥖🥖                                                                                           
            xxxxxxxxxxxxxxxxxxx
            "
            .into(),
            Pointers: vec![Pointer {
                target: AisleID(15),
                triggers: vec!["straight".into()],
                message: None,
            }],
        },
        Aisle {
            name: "Aisle RUN".into(),
            desc: " 
            xxxxxxxxxxxxxxxxxxx
            🌂🎒🕶️🌂🎒🕶️🥮🥮
            🌂🎒🕶️🎂🥐🌂🎒🕶️
            🥐🌂🎒🕶️🥖🌂🎒🕶️                                                                               
            xxxxxxxxxxxxxxxxxxx
            "
            .into(),
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
            name: "Aisle FOR".into(),
            desc: " 
            xxxxxxxxxxxxxxxxxxx
            🍫🍭💐🌂🎒🍫🍭💐
            🌂🎒🍫🍭💐🍫🍭💐
            🥐🍫🍭💐🥖🍫🍭💐                                                                               
            xxxxxxxxxxxxxxxxxxx
            "
            .into(),
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
            name: "Aisle LIFE".into(),
            desc: " 
            xxxxxxxxxxxxxxxxxxx
            🦕🦕🦕🦕🧃🧃🧃🧃
            🫐🫐🦕🦕🦕🦕🦕🫐
            🥑🥑🥑🥑🥑🦖🦖🦖                                                                                         
            xxxxxxxxxxxxxxxxxxx
            "
            .into(),
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
    let end_aisles = [AisleID(15)];
    let mut input = String::new();
    let mut input_one = String::new();

    let mut at = AisleID(0);
    println!();
    println!();
    println!("The Ultimate Walmart Searcharcha Hunt");
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
    println!("=======================================================");
    //playing the intro music here
    let intro_time: u64 = 5;
    let mp3_file_path = "./src/beep.mp3";
    play_mp3_file(mp3_file_path, &intro_time);
    println!();

    println!();
    let text = "You are grocery shopping with your parents!! WOW, so fun...";
    let delay = Duration::from_millis(80); // Adjust the delay as needed
    let mut stdout = io::stdout();

    for char in text.chars() {
        print!("{}", char);
        stdout.flush().unwrap(); // Flush stdout to ensure characters are displayed immediately
        sleep(delay);
    }
    println!();

    //sleep(Duration::from_secs(2));
    let text = "Except, now you are stressed. You reached the billing counter only for your parents to realize THEY FORGOT THE SRIRACHA!!";
    let delay = Duration::from_millis(70); // Adjust the delay as needed
    let mut stdout = io::stdout();

    for char in text.chars() {
        print!("{}", char);
        stdout.flush().unwrap(); // Flush stdout to ensure characters are displayed immediately
        sleep(delay);
    }
    println!();

    let text = "Your mission for this game is to grab the sriracha before the cashier checks out all of the items and before your parents leave..... without you.... :'(";
    let delay = Duration::from_millis(70); // Adjust the delay as needed
    let mut stdout = io::stdout();

    for char in text.chars() {
        print!("{}", char);
        stdout.flush().unwrap(); // Flush stdout to ensure characters are displayed immediately
        sleep(delay);
    }
    println!();

    //playing the intro music here
    let intro_time: u64 = 5;
    let mp3_file_path = "./src/beep.mp3";
    play_mp3_file(mp3_file_path, &intro_time);
    println!();

    let str1 = "The cashier has started billing.\nThe beeping has started.\nWill you be able to find the Sriracha? And bring it back in time?";
    let delay = Duration::from_millis(2500); // Adjust the delay as needed
    let mut stdout = io::stdout();

    for line in str1.lines() {
        println!("{}", line);
        stdout.flush().unwrap(); // Flush stdout to ensure characters are displayed immediately
        sleep(delay);
    }
    println!();
    println!("Ready...?");
    println!();

    //playing the intro music here
    let intro_time: u64 = 5;
    let mp3_file_path = "./src/beep.mp3";
    play_mp3_file(mp3_file_path, &intro_time);

    let start = Instant::now();
    let time_limit: u64 = 2400;

    loop {
        // We don't want to move out of Aisles, so we take a reference
        let here = &Aisles[at.0];

        //prints the aisle name and description
        println!();
        println!("++++++++++++++++++++++++++++++++++++++++++++++++++");

        //the game should end when the time ends
        let mut new_now = Instant::now();
        println!();
        println!(
            ">You have {:?} seconds left!!!<",
            Duration::from_secs(time_limit).as_secs() - new_now.duration_since(start).as_secs()
        );

        if new_now.duration_since(start) >= Duration::from_secs(time_limit) {
            println!(
                "
            =====================================================================================
            =====================================================================================
                                              # GAME OVER ⌛️ #
             You ran out of time :( Your parents left you..... Sriracha blood is all over you >:)
            =====================================================================================
            =====================================================================================)"
            );
            break;
        }
        println!();
        println!("{}\n{}", here.name, here.desc);

        if end_aisles.contains(&at) {
            println!(
                "
            =====================================================================================
            =====================================================================================
                                                # YOU WIN #
            You saved the day for the family! You can now go home safely with your parents :)
            =====================================================================================
            =====================================================================================
            "
            );
            break;
        }

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
                println!("Is the sriracha in this aisle?: 'Yes' or 'No'?");
                io::stdout().flush().unwrap();
                input_one.clear();
                io::stdin().read_line(&mut input_one).unwrap();
                let input_one = input_one.trim();
                //if (at == )
                if input_one == "No" || input_one == "no" {
                    if at == AisleID(7) {
                        at = AisleID(0);
                        println!("Darn! You missed it… back to the cashier");
                        keep = false;
                    }
                    break;
                } else if input_one == "Yes" || input_one == "yes" {
                    // say yes
                    if at != AisleID(7) {
                        at = AisleID(0);
                        println!("Darn! You missed it… back to the cashier");
                        keep = false;
                    } else {
                        println!("You found it! Hurry and navigate your way back to the cashier from level 2!");
                    }
                    break;
                } else {
                    println!("You can't do that!");
                }
            }
        }

        if keep {
            loop {
                println!();
                print!("What will you do?\n");
                show_moves(here);

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
}

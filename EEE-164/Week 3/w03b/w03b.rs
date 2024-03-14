use std::io::*;

pub trait Media {
    fn play (&self);
    fn title (&self) -> String;
    fn artist (&self) -> String;
}

struct Song {
    title:  String,
    artist: String,
}

impl Media for Song {
    fn play(&self) {
        println!("Now playing: {} by {}", self.title, self.artist);
    }
    fn title(&self) -> String {
        self.title.clone()
    }
    fn artist(&self) -> String {
        self.artist.clone()
    }
}

struct Queue<T> {
    list: Vec<T>,
}

impl <T: Media> Queue <T> {
    fn new() -> Self {
        Queue { list: Vec::new() }
    }
    fn play(&mut self) {
        if !self.list.is_empty() {
            let media = self.list.remove(0);
            media.play();
        } else {
            println!("Queue is empty! No media to play...");
        }
    }
    fn add(&mut self, media: T) {
        if self.list.len() < 12 {
            println!("Successfully added {} by {} to the queue!", media.title(), media.artist());
            self.list.push(media);
        } else {
            println!("Queue is full! {} by {} is dropped.", media.title(), media.artist());
        }
    }
    fn show_queue(&self) {
        if self.list.is_empty() {
            println!("No media in queue.");
        } else {
            println!("-----mEEEdia bot-----");
            for (idx, media) in self.list.iter().enumerate() {
                println!("{}. {} by {}", idx + 1, media.title(), media.artist());
            }
            println!("---------------------");
        }
    }
    fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

fn main() {
    let mut str_in = String::new();
    stdin().read_line(&mut str_in).expect("Invalid input!");
    let test_case: usize = str_in.trim().parse().expect("Not an integer!"); 

    let mut playlist = Queue::new();
    // Iterate each Test Case
    for i in 1..=test_case {
        str_in.clear();
        stdin().read_line(&mut str_in).expect("Invalid input!");
        let split_in: Vec <&str> = str_in.split_whitespace().collect();

        
        match split_in[0] {
            "play" => {
                playlist.play();
            },
            "add" => {
                let title = split_in[1..].join(" "); // Joining back the title in case it was split
                let artist = match title.as_str() {
                    "OMG" => "New Jeans",
                    "Perfect Night" => "LE SSERAFIM",
                    "Raining in Manila" => "Lola Amour",
                    "Never Gonna Give You Up" => "Rick Astley",
                    "Mananatili" => "Cup of Joe",
                    "Aphrodite" => "The Ridleys",
                    "Hanggang sa Buwan" => "Kenaniah",
                    "Dumaloy" => "SUD",
                    &_ => todo!(),
                };
                playlist.add(Song { title: title.to_string(), artist: artist.to_string() });
            },
            "show_queue" => {
                playlist.show_queue();
            },
            _ => (),
        }
    }
}

// cat in_w03b_pub_s01.txt | ./w03b.exe | Out-File -Encoding UTF8 output.txt
// Compare-Object (gc output.txt) (gc out_w03b_pub_s01.txt)

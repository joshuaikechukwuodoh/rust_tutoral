fn main() {
    // Create an empty playlist (vector)
    let mut playlist = Vec::new(); 

    // Add some songs to the playlist
    playlist.push("Song 1");
    playlist.push("Song 2");
    playlist.push("Song 3");

    // Show all the songs in the playlist
    for song in &playlist {
        println!("Playing: {}", song);
    }

    // Remove the last song from the playlist
    playlist.pop();

    println!("Updated playlist:");
    for song in &playlist {
        println!("Playing: {}", song);
    }
}

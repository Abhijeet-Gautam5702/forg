// CLI TOOL
// forg --init => create a .forg/forg.config file
// sample forg.config file
/*
 * [
 *  {
 *      "pattern": "*.png",
 *      "path": "/home/abhijeet/Pictures/Screenshots"
 *  },
 *  {
 *      "pattern": "*.txt",
 *      "path": "/home/abhijeet/Documents"
 *  },
 * ]
 *
 */
// forg --exec ./Downloads
// - scans ./Downloads directory
// - move all the files (not directories) to their respective paths

fn main() {
    println!("Hello, world!");
}

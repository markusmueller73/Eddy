//! Eddy
//! A lightweight text editor for the terminal.
//! Written in Rust for Linux, MacOS and Windows.
//! (c) 2026 by Markus Müller
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.
#[macro_use]
pub mod logger;
mod editor;

fn main() -> Result<(), i32> {
    // Start selfmade small file logging
    crate::logger::DebugLog::init();
    // Get the first command line argument, it is by default the name of the executeable
    let exe = std::env::args().next().unwrap_or(crate::editor::TITLE.to_ascii_lowercase());
    // Create a String vector from the rest of the command line arguments and check if any
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let mut filename = String::new();
    if !argv.is_empty() {
        for arg in argv {
            if arg.starts_with("-") {
                // Check command line parameter
                match arg.as_str() {
                    "-h" | "--help" => {
                        println!("Usage: {} [options] [file]", exe);
                        println!("Options:");
                        println!("  -h, --help    Display this help message");
                        println!("  -V, --version Show the version number");
                        std::process::exit(0);
                    }
                    "-V" | "--version" => {
                        println!("{} Version {}", crate::editor::TITLE, crate::editor::VERSION);
                        std::process::exit(0);
                    }
                    _ => {
                        println!("Unknown option: {}", arg);
                        std::process::exit(0);
                    }
                }
            } else {
                // Append all arguments, the user may type the filename with spaces and without (double)quotes
                filename.push_str(&arg);
            }
        }
    }
    // Run the text editor
    crate::editor::text_editor::TextEditor::run(&filename)
}

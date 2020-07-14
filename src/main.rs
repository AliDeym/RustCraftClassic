/*
    Copyright (c) 2020 Ali Deym

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
*/

use std::env;
use std::thread;

use rcclassic::core::{Core, Network};

fn main() {
    // Thread size.
    let env_threadsize = env::var("THREADSIZE").unwrap_or(String::from("0")); // Defaulting to zero. Core will handle the case and set threadsize to core count.
    let threadsize = env_threadsize.parse::<usize>().unwrap_or(0);

    // Instantiate a core struct.
    let mut core = Core::new(threadsize);

    // Initialize memory channels.
    core.generate_mem_chans();

    // Move receiver into main thread to handle receiving network packets.
    //let main_receiver = core.take_receiver();
    core.network_listen();

    
    core.handle_received_packets();
}

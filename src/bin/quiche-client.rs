// Copyright (C) 2020, Cloudflare, Inc.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//
//     * Redistributions in binary form must reproduce the above copyright
//       notice, this list of conditions and the following disclaimer in the
//       documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
// IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
// PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// TODO
// 1. make new application protocol to start implementing SSH like behavior
// 2. determine how to capture keypresses to pass to server (nonprintable, such as ^C, ^\, ^Z, etc.)
// 3. Figure out multiplexing. 
//      need to check for existence of shared socket before making connection to remote.
//      need to make AF_UNIX socket at `/tmp/%u@%h:%p` to accept new connections on the shared connection if 
//          it doesnt already exist.
//      need to connect to existing shared socket if it does exist, and send data to that socket. whomever 
//          started that shared socket 
//          then also needs to handle passing data from that shared socket to the real one.
//      need to then figure out how to get the data back to the appropriate application which is connected 
//          with the shared socket...
//          I would be tempted to make a new unix socket for each other app, but SSH does it with just the one.
//      also verify all connections made over the UNIX socket are from the same user (use SO_PEERCRED) to get 
//          peer's UID
//      see sections 5,6 of RFC 4254
//
// 4. add support for requesting just a single command (e.g. SQSH user@machine ls)
// 5. add support for requesting a complete shell based on the default shell of the user being logged into.
// 6. add support for window resizing when using a shell
// 7. add control character filtering (rfc 9251 sec. 9.2) 

// figure out certificate/keying process to ensure only authorized users can access...


use SQSH::args::*;

use SQSH::common::*;

use SQSH::client::*;

fn main() {
    env_logger::builder().format_timestamp_nanos().init();

    // Parse CLI parameters.
    let docopt = docopt::Docopt::new(CLIENT_USAGE).unwrap();
    let conn_args = CommonArgs::with_docopt(&docopt);
    let args = ClientArgs::with_docopt(&docopt);

    match connect(args, conn_args, stdout_sink) {
        Err(ClientError::HandshakeFail) => std::process::exit(-1),

        Err(ClientError::HttpFail) => std::process::exit(-2),

        Err(ClientError::Other(e)) => panic!("{}", e),

        Ok(_) => (),
    }
}

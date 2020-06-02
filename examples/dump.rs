/*
    Copyright (c) 2020, ZomboDB, LLC

    Permission to use, copy, modify, and distribute this software and its documentation for any purpose, without fee, and
    without a written agreement is hereby granted, provided that the above copyright notice and this paragraph and the
    following two paragraphs appear in all copies.

    IN NO EVENT SHALL ZomboDB, LLC BE LIABLE TO ANY PARTY FOR DIRECT, INDIRECT, SPECIAL, INCIDENTAL, OR CONSEQUENTIAL
    DAMAGES, INCLUDING LOST PROFITS, ARISING OUT OF THE USE OF THIS SOFTWARE AND ITS DOCUMENTATION, EVEN IF ZomboDB, LLC
    HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

    ZomboDB, LLC SPECIFICALLY DISCLAIMS ANY WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
    MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE SOFTWARE PROVIDED HEREUNDER IS ON AN "AS IS" BASIS, AND
    ZomboDB, LLC HAS NO OBLIGATIONS TO PROVIDE MAINTENANCE, SUPPORT, UPDATES, ENHANCEMENTS, OR MODIFICATIONS.
*/
use postgres_parser::*;

/// A simple CLI example that parses its first argument (as an SQL statement) and outputs
/// the resulting parse tree in Rust's standard "Debug" output
///
/// ## Usage
///
/// ```shell
/// $ target/debug/examples/dump "SELECT 1"
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let parse_list = parse_query(args.get(1).expect("no arguments")).unwrap();

    println!("{:#?}", parse_list);
}
